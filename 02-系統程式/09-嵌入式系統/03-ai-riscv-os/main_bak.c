/*
 * RISC-V 嵌入式多任務作業系統 (RTOS)
 * 支援多執行緒、搶佔式排程、Context Switching
 */

#include <stdint.h>
#include <stddef.h>

/* ==================== UART 驅動 ==================== */
#define UART_BASE 0x10000000
#define UART_THR (UART_BASE + 0x00)  // Transmit Holding Register
#define UART_LSR (UART_BASE + 0x05)  // Line Status Register

// UART 輸出一個字元
void uart_putc(char c) {
    volatile uint8_t *uart_thr = (uint8_t *)UART_THR;
    *uart_thr = c;
}

// UART 輸出字串
void uart_puts(const char *s) {
    while (*s) {
        if (*s == '\n') {
            uart_putc('\r');  // 加入 CR 以正確換行
        }
        uart_putc(*s++);
    }
}

// 簡單的整數轉字串
void print_int(int val) {
    if (val < 0) {
        uart_putc('-');
        val = -val;
    }
    
    char buf[20];
    int i = 0;
    
    if (val == 0) {
        uart_putc('0');
        return;
    }
    
    while (val > 0) {
        buf[i++] = '0' + (val % 10);
        val /= 10;
    }
    
    while (i > 0) {
        uart_putc(buf[--i]);
    }
}

// 簡化版 printf
void uart_printf(const char *fmt, ...) {
    // 暫時不使用可變參數，改用直接傳參
    uart_puts(fmt);
}

/* ==================== 系統配置 ==================== */
#define MAX_TASKS 8
#define STACK_SIZE 1024
#define TIME_SLICE 10  // 時間片（單位：timer ticks）

/* ==================== RISC-V CSR 定義 ==================== */
#define read_csr(reg) ({ \
    unsigned long __tmp; \
    asm volatile ("csrr %0, " #reg : "=r"(__tmp)); \
    __tmp; })

#define write_csr(reg, val) ({ \
    asm volatile ("csrw " #reg ", %0" :: "r"(val)); })

#define set_csr(reg, bit) ({ \
    asm volatile ("csrs " #reg ", %0" :: "r"(bit)); })

#define clear_csr(reg, bit) ({ \
    asm volatile ("csrc " #reg ", %0" :: "r"(bit)); })

/* ==================== 任務狀態 ==================== */
typedef enum {
    TASK_READY,
    TASK_RUNNING,
    TASK_BLOCKED,
    TASK_TERMINATED
} task_state_t;

/* ==================== 任務控制區塊 (TCB) ==================== */
typedef struct {
    unsigned long *sp;         // 堆疊指標 (64-bit)
    unsigned long stack[STACK_SIZE]; // 任務堆疊 (64-bit)
    task_state_t state;        // 任務狀態
    uint32_t priority;         // 優先權
    uint32_t time_slice;       // 剩餘時間片
    void (*entry)(void *);     // 任務進入點
    void *arg;                 // 任務參數
    uint32_t tid;              // 任務 ID
} tcb_t;

/* ==================== 全域變數 ==================== */
static tcb_t tasks[MAX_TASKS];
static uint32_t current_task = 0;
static uint32_t task_count = 0;
static volatile uint32_t system_ticks = 0;

/* ==================== Context Switching ==================== */
// 保存/恢復暫存器的宏 (64-bit)
#define SAVE_CONTEXT() \
    asm volatile ( \
        "addi sp, sp, -256\n"      /* 分配堆疊空間 (64-bit 需要更多) */ \
        "sd ra, 0(sp)\n"           /* 保存返回位址 */ \
        "sd t0, 8(sp)\n" \
        "sd t1, 16(sp)\n" \
        "sd t2, 24(sp)\n" \
        "sd t3, 32(sp)\n" \
        "sd t4, 40(sp)\n" \
        "sd t5, 48(sp)\n" \
        "sd t6, 56(sp)\n" \
        "sd a0, 64(sp)\n" \
        "sd a1, 72(sp)\n" \
        "sd a2, 80(sp)\n" \
        "sd a3, 88(sp)\n" \
        "sd a4, 96(sp)\n" \
        "sd a5, 104(sp)\n" \
        "sd a6, 112(sp)\n" \
        "sd a7, 120(sp)\n" \
        "sd s0, 128(sp)\n" \
        "sd s1, 136(sp)\n" \
        "sd s2, 144(sp)\n" \
        "sd s3, 152(sp)\n" \
        "sd s4, 160(sp)\n" \
        "sd s5, 168(sp)\n" \
        "sd s6, 176(sp)\n" \
        "sd s7, 184(sp)\n" \
        "sd s8, 192(sp)\n" \
        "sd s9, 200(sp)\n" \
        "sd s10, 208(sp)\n" \
        "sd s11, 216(sp)\n" \
    )

#define RESTORE_CONTEXT() \
    asm volatile ( \
        "ld ra, 0(sp)\n" \
        "ld t0, 8(sp)\n" \
        "ld t1, 16(sp)\n" \
        "ld t2, 24(sp)\n" \
        "ld t3, 32(sp)\n" \
        "ld t4, 40(sp)\n" \
        "ld t5, 48(sp)\n" \
        "ld t6, 56(sp)\n" \
        "ld a0, 64(sp)\n" \
        "ld a1, 72(sp)\n" \
        "ld a2, 80(sp)\n" \
        "ld a3, 88(sp)\n" \
        "ld a4, 96(sp)\n" \
        "ld a5, 104(sp)\n" \
        "ld a6, 112(sp)\n" \
        "ld a7, 120(sp)\n" \
        "ld s0, 128(sp)\n" \
        "ld s1, 136(sp)\n" \
        "ld s2, 144(sp)\n" \
        "ld s3, 152(sp)\n" \
        "ld s4, 160(sp)\n" \
        "ld s5, 168(sp)\n" \
        "ld s6, 176(sp)\n" \
        "ld s7, 184(sp)\n" \
        "ld s8, 192(sp)\n" \
        "ld s9, 200(sp)\n" \
        "ld s10, 208(sp)\n" \
        "ld s11, 216(sp)\n" \
        "addi sp, sp, 256\n" \
    )

/* ==================== 排程器 ==================== */
// 找下一個可執行的任務（Round-Robin with Priority）
static uint32_t scheduler(void) {
    uint32_t next = current_task;
    uint32_t highest_priority = 0;
    int found = 0;
    
    // 優先權排程：找最高優先權的 READY 任務
    for (uint32_t i = 0; i < task_count; i++) {
        if (tasks[i].state == TASK_READY && 
            tasks[i].priority > highest_priority) {
            highest_priority = tasks[i].priority;
            next = i;
            found = 1;
        }
    }
    
    // 如果沒找到，使用 Round-Robin
    if (!found) {
        for (uint32_t i = 1; i <= task_count; i++) {
            uint32_t idx = (current_task + i) % task_count;
            if (tasks[idx].state == TASK_READY) {
                next = idx;
                break;
            }
        }
    }
    
    return next;
}

// Context Switch
void context_switch(void) {
    // 保存當前任務的 SP
    register unsigned long sp_val asm("sp");
    tasks[current_task].sp = (unsigned long *)sp_val;
    
    // 如果當前任務還在運行，改為 READY
    if (tasks[current_task].state == TASK_RUNNING) {
        tasks[current_task].state = TASK_READY;
    }
    
    // 選擇下一個任務
    current_task = scheduler();
    tasks[current_task].state = TASK_RUNNING;
    tasks[current_task].time_slice = TIME_SLICE;
    
    // 恢復新任務的 SP
    sp_val = (unsigned long)tasks[current_task].sp;
    asm volatile("mv sp, %0" : : "r"(sp_val));
}

/* ==================== Timer 中斷處理 ==================== */
void timer_interrupt_handler(void) {
    system_ticks++;
    
    uart_puts("[IRQ] Timer tick=");
    print_int(system_ticks);
    uart_puts(", current_task=");
    print_int(current_task);
    uart_puts("\n");
    
    // 時間片用完，觸發任務切換
    if (tasks[current_task].time_slice > 0) {
        tasks[current_task].time_slice--;
    }
    
    if (tasks[current_task].time_slice == 0) {
        uart_puts("[IRQ] Time slice expired, switching task...\n");
        // 這裡應該要做 context switch，但先簡單處理
        // 找下一個 READY 的任務
        uint32_t next = (current_task + 1) % task_count;
        while (tasks[next].state != TASK_READY && next != current_task) {
            next = (next + 1) % task_count;
        }
        
        if (next != current_task && tasks[next].state == TASK_READY) {
            tasks[current_task].state = TASK_READY;
            current_task = next;
            tasks[current_task].state = TASK_RUNNING;
            tasks[current_task].time_slice = TIME_SLICE;
            uart_puts("[IRQ] Switched to task ");
            print_int(current_task);
            uart_puts("\n");
        }
    }
}

/* ==================== 任務管理 ==================== */
// 初始化任務堆疊
static void init_task_stack(tcb_t *task) {
    unsigned long *stk = &task->stack[STACK_SIZE - 1];
    
    // 設定初始暫存器值（模擬中斷返回）
    *(--stk) = (unsigned long)task->entry;  // ra: 返回位址（任務進入點）
    *(--stk) = 0;  // t0
    *(--stk) = 0;  // t1
    *(--stk) = 0;  // t2
    *(--stk) = 0;  // t3
    *(--stk) = 0;  // t4
    *(--stk) = 0;  // t5
    *(--stk) = 0;  // t6
    *(--stk) = (unsigned long)task->arg;  // a0: 第一個參數
    *(--stk) = 0;  // a1
    *(--stk) = 0;  // a2
    *(--stk) = 0;  // a3
    *(--stk) = 0;  // a4
    *(--stk) = 0;  // a5
    *(--stk) = 0;  // a6
    *(--stk) = 0;  // a7
    *(--stk) = 0;  // s0-s11
    *(--stk) = 0;
    *(--stk) = 0;
    *(--stk) = 0;
    *(--stk) = 0;
    *(--stk) = 0;
    *(--stk) = 0;
    *(--stk) = 0;
    *(--stk) = 0;
    *(--stk) = 0;
    *(--stk) = 0;
    *(--stk) = 0;
    
    task->sp = stk;
}

// 創建任務
int task_create(void (*entry)(void *), void *arg, uint32_t priority) {
    if (task_count >= MAX_TASKS) {
        return -1;  // 任務數已滿
    }
    
    tcb_t *task = &tasks[task_count];
    task->entry = entry;
    task->arg = arg;
    task->state = TASK_READY;
    task->priority = priority;
    task->time_slice = TIME_SLICE;
    task->tid = task_count;
    
    init_task_stack(task);
    
    int tid = task_count;  // 先保存當前的 task_count
    task_count++;          // 然後才增加
    return tid;            // 返回保存的值
}

// 任務讓出 CPU
void task_yield(void) {
    SAVE_CONTEXT();
    context_switch();
    RESTORE_CONTEXT();
}

// 任務延遲（簡化版）
void task_delay(uint32_t ticks) {
    uint32_t start = system_ticks;
    while ((system_ticks - start) < ticks) {
        task_yield();
    }
}

/* ==================== 系統初始化 ==================== */
void os_init(void) {
    task_count = 0;
    current_task = 0;
    system_ticks = 0;
    
    // 初始化所有任務為 TERMINATED
    for (int i = 0; i < MAX_TASKS; i++) {
        tasks[i].state = TASK_TERMINATED;
    }
    
    uart_puts("\n=================================\n");
    uart_puts("  RISC-V RTOS Initializing...\n");
    uart_puts("=================================\n");
}

// 宣告 startup.S 中的函數
extern void init_timer(void);
extern void reset_timer(void);

// 初始化並測試 Timer
void test_timer(void) {
    uart_puts("\n[DEBUG] Testing timer interrupt...\n");
    uart_puts("[DEBUG] Waiting for timer ticks...\n");
    
    uint32_t start_ticks = system_ticks;
    // 等待一段時間看 timer 是否觸發
    for (volatile int i = 0; i < 10000000; i++);
    
    uart_puts("[DEBUG] After wait: ticks=");
    print_int(system_ticks);
    uart_puts(" (should be > ");
    print_int(start_ticks);
    uart_puts(")\n");
    
    if (system_ticks == start_ticks) {
        uart_puts("[WARNING] Timer interrupt not working!\n");
    } else {
        uart_puts("[SUCCESS] Timer interrupt is working!\n");
    }
    uart_puts("\n");
}

// 啟動作業系統
void os_start(void) {
    if (task_count == 0) {
        uart_puts("[ERROR] No tasks to run!\n");
        return;  // 沒有任務可執行
    }
    
    uart_puts("[INFO] Starting RTOS with ");
    print_int(task_count);
    uart_puts(" tasks\n");
    uart_puts("[INFO] Scheduler running...\n\n");
    
    // 設定第一個任務
    current_task = 0;
    tasks[current_task].state = TASK_RUNNING;
    
    uart_puts("[DEBUG] Task 0 entry point: 0x");
    // 簡單的 hex 輸出
    unsigned long entry_addr = (unsigned long)tasks[current_task].entry;
    for (int i = 15; i >= 0; i--) {
        int digit = (entry_addr >> (i * 4)) & 0xF;
        uart_putc(digit < 10 ? '0' + digit : 'a' + digit - 10);
    }
    uart_puts("\n");
    
    uart_puts("[DEBUG] Jumping to first task...\n");
    
    // 直接呼叫第一個任務（不使用 context switch）
    tasks[current_task].entry(tasks[current_task].arg);
    
    // 如果任務返回（所有任務完成）
    uart_puts("\n[SUCCESS] RTOS execution completed normally.\n");
    uart_puts("[INFO] System halted.\n");
}

/* ==================== 示範任務 ==================== */
void task1(void *arg) {
    uart_puts("[TASK1] Started (Priority 1)\n");
    
    for (int count = 1; count <= 3; count++) {
        uart_puts("[TASK1] Running... count=");
        print_int(count);
        uart_puts("\n");
        
        // 簡單的忙等待（不依賴 timer）
        for (volatile int i = 0; i < 1000000; i++);
    }
    
    uart_puts("[TASK1] Finished\n");
    tasks[0].state = TASK_TERMINATED;
    
    // 切換到下一個任務
    if (task_count > 1) {
        current_task = 1;
        tasks[1].state = TASK_RUNNING;
        tasks[1].entry(tasks[1].arg);
    }
}

void task2(void *arg) {
    uart_puts("[TASK2] Started (Priority 2)\n");
    
    for (int count = 1; count <= 3; count++) {
        uart_puts("[TASK2] Running... count=");
        print_int(count);
        uart_puts("\n");
        
        for (volatile int i = 0; i < 1000000; i++);
    }
    
    uart_puts("[TASK2] Finished\n");
    tasks[1].state = TASK_TERMINATED;
    
    // 切換到下一個任務
    if (task_count > 2) {
        current_task = 2;
        tasks[2].state = TASK_RUNNING;
        tasks[2].entry(tasks[2].arg);
    }
}

void task3(void *arg) {
    uart_puts("[TASK3] Started (Priority 1)\n");
    
    for (int count = 1; count <= 3; count++) {
        uart_puts("[TASK3] Running... count=");
        print_int(count);
        uart_puts("\n");
        
        for (volatile int i = 0; i < 1000000; i++);
    }
    
    uart_puts("[TASK3] Finished\n");
    tasks[2].state = TASK_TERMINATED;
    
    uart_puts("\n[INFO] All tasks completed!\n");
}

/* ==================== 主程式 ==================== */
int main(void) {
    // 初始化作業系統
    os_init();
    
    // 測試 Timer 中斷
    test_timer();
    
    uart_puts("[INFO] Creating tasks...\n");
    
    // 創建任務
    int tid1 = task_create(task1, NULL, 1);
    uart_puts("[INFO] Created Task 1, TID=");
    print_int(tid1);
    uart_puts("\n");
    
    int tid2 = task_create(task2, NULL, 2);
    uart_puts("[INFO] Created Task 2, TID=");
    print_int(tid2);
    uart_puts("\n");
    
    int tid3 = task_create(task3, NULL, 1);
    uart_puts("[INFO] Created Task 3, TID=");
    print_int(tid3);
    uart_puts("\n");
    
    uart_puts("[INFO] Total tasks: ");
    print_int(task_count);
    uart_puts("\n\n");
    
    // 啟動作業系統
    os_start();
    
    // 永遠不會到這裡
    while (1);
    
    return 0;
}