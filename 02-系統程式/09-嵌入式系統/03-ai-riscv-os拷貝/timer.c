
/*
 * Timer 初始化函數 (C wrapper)
 */

extern void init_timer(void);
extern void reset_timer(void);

// 在 main.c 的 os_start() 之前呼叫
void setup_timer(void) {
    init_timer();
}