#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include <stdint.h>
#include <stdbool.h>

#define MAX_TOKENS 4096
#define MAX_QUADS 4096
#define MAX_STRINGS 1024
#define STR_LEN 64

// =========================================================
// 1. 核心資料結構：動態型別系統 (Tagged Union)
// =========================================================
typedef enum { VAL_NIL, VAL_INT, VAL_STR, VAL_ARRAY, VAL_DICT } ValueType;

struct Value;
struct Array;
struct Dict;

typedef struct Value {
    ValueType type;
    union {
        int64_t i;
        char* s;
        struct Array* arr;
        struct Dict* dict;
    } as;
} Value;

// 動態陣列
typedef struct Array {
    Value* items;
    int count;
    int capacity;
} Array;

// 雜湊表節點 (鏈結串列實作)
typedef struct HashNode {
    char key[STR_LEN];
    Value val;
    struct HashNode* next;
} HashNode;

typedef struct Dict {
    HashNode** buckets;
    int capacity;
} Dict;

// 建立基本型別
Value make_nil() { Value v; v.type = VAL_NIL; return v; }
Value make_int(int64_t i) { Value v; v.type = VAL_INT; v.as.i = i; return v; }
Value make_str(const char* s) { Value v; v.type = VAL_STR; v.as.s = strdup(s); return v; }

// =========================================================
// 2. 集合操作實作 (Array & Dict)
// =========================================================
Array* new_array() {
    Array* arr = malloc(sizeof(Array));
    arr->capacity = 8;
    arr->count = 0;
    arr->items = malloc(sizeof(Value) * arr->capacity);
    return arr;
}

void array_push(Array* arr, Value val) {
    if (arr->count >= arr->capacity) {
        arr->capacity *= 2;
        arr->items = realloc(arr->items, sizeof(Value) * arr->capacity);
    }
    arr->items[arr->count++] = val;
}

Dict* new_dict() {
    Dict* dict = malloc(sizeof(Dict));
    dict->capacity = 16;
    dict->buckets = calloc(dict->capacity, sizeof(HashNode*));
    return dict;
}

unsigned int hash_func(const char* key) {
    unsigned int hash = 5381;
    int c;
    while ((c = *key++)) hash = ((hash << 5) + hash) + c; // djb2
    return hash;
}

void dict_set(Dict* dict, const char* key, Value val) {
    int idx = hash_func(key) % dict->capacity;
    HashNode* node = dict->buckets[idx];
    while (node) {
        if (strcmp(node->key, key) == 0) { node->val = val; return; }
        node = node->next;
    }
    HashNode* new_node = malloc(sizeof(HashNode));
    strncpy(new_node->key, key, STR_LEN - 1);
    new_node->val = val;
    new_node->next = dict->buckets[idx];
    dict->buckets[idx] = new_node;
}

Value dict_get(Dict* dict, const char* key) {
    int idx = hash_func(key) % dict->capacity;
    HashNode* node = dict->buckets[idx];
    while (node) {
        if (strcmp(node->key, key) == 0) return node->val;
        node = node->next;
    }
    return make_nil();
}

// =========================================================
// 3. 詞法分析 (Lexer) 與 中間碼 (Quads)
// =========================================================
typedef enum {
    OP_IMM, OP_LOAD_STR, OP_ADD, OP_SUB, OP_MUL, OP_DIV, OP_CMP_EQ, OP_CMP_LT, OP_CMP_GT,
    OP_STORE, OP_NEW_ARR, OP_NEW_DICT, OP_APPEND_ITEM, OP_SET_ITEM, OP_GET_ITEM,
    OP_JMP, OP_JMP_F, OP_PARAM, OP_CALL, OP_FORMAL, OP_RET_VAL, OP_PRINT_VAL, OP_PRINT_NL,
    OP_FUNC_BEG, OP_FUNC_END
} OpCode;

typedef struct {
    OpCode op;
    char arg1[STR_LEN];
    char arg2[STR_LEN];
    char res[STR_LEN];
} Quad;

// 為節省篇幅，此處省略了完整的遞迴下降 Parser。
// 在 C 語言中，通常會使用 Flex/Bison 或手寫狀態機來產生 Quads。
// 此結構直接對接前述邏輯生成的 Quads。

// =========================================================
// 4. 虛擬機 (Virtual Machine)
// =========================================================
typedef struct Frame {
    Dict* vars;
    int ret_pc;
    char ret_var[STR_LEN];
    Value* incoming_args;
    int incoming_count;
    int formal_idx;
} Frame;

Quad quads[MAX_QUADS];
int quads_count = 0;
char* string_pool[MAX_STRINGS];
int string_pool_count = 0;

Frame stack[256];
int sp = 0;

Value get_var(const char* name) {
    if (name[0] == '-' && name[1] == '\0') return make_int(0);
    if (isdigit(name[0]) || (name[0] == '-' && isdigit(name[1]))) return make_int(atoll(name));
    return dict_get(stack[sp].vars, name);
}

void set_var(const char* name, Value val) {
    if (strcmp(name, "-") == 0 || strcmp(name, "?") == 0) return;
    dict_set(stack[sp].vars, name, val);
}

void vm_run() {
    int pc = 0;
    Value param_stack[256];
    int param_sp = 0;
    
    // 初始化全域 Frame
    stack[sp].vars = new_dict();
    
    // 預先掃描函數位置
    Dict* func_map = new_dict();
    for (int i = 0; i < quads_count; i++) {
        if (quads[i].op == OP_FUNC_BEG) {
            dict_set(func_map, quads[i].arg1, make_int(i + 1));
        }
    }

    printf("\n=== VM 執行開始 ===\n");
    while (pc < quads_count) {
        Quad* q = &quads[pc];
        switch (q->op) {
            case OP_FUNC_BEG:
                while (quads[pc].op != OP_FUNC_END) pc++;
                break;
            case OP_IMM: set_var(q->res, make_int(atoll(q->arg1))); break;
            case OP_LOAD_STR: set_var(q->res, make_str(string_pool[atoi(q->arg1)])); break;
            case OP_ADD: set_var(q->res, make_int(get_var(q->arg1).as.i + get_var(q->arg2).as.i)); break;
            case OP_SUB: set_var(q->res, make_int(get_var(q->arg1).as.i - get_var(q->arg2).as.i)); break;
            case OP_MUL: set_var(q->res, make_int(get_var(q->arg1).as.i * get_var(q->arg2).as.i)); break;
            case OP_DIV: set_var(q->res, make_int(get_var(q->arg1).as.i / (get_var(q->arg2).as.i ? get_var(q->arg2).as.i : 1))); break;
            
            case OP_STORE: set_var(q->res, get_var(q->arg1)); break;
            case OP_NEW_ARR: set_var(q->res, (Value){.type = VAL_ARRAY, .as.arr = new_array()}); break;
            case OP_NEW_DICT: set_var(q->res, (Value){.type = VAL_DICT, .as.dict = new_dict()}); break;
            
            case OP_APPEND_ITEM: {
                Value arr_val = get_var(q->arg1);
                if (arr_val.type == VAL_ARRAY) array_push(arr_val.as.arr, get_var(q->res));
                break;
            }
            case OP_SET_ITEM: {
                Value obj = get_var(q->arg1);
                Value val = get_var(q->res);
                if (obj.type == VAL_ARRAY) {
                    obj.as.arr->items[get_var(q->arg2).as.i] = val;
                } else if (obj.type == VAL_DICT) {
                    dict_set(obj.as.dict, get_var(q->arg2).as.s, val);
                }
                break;
            }
            case OP_GET_ITEM: {
                Value obj = get_var(q->arg1);
                Value res = make_nil();
                if (obj.type == VAL_ARRAY) {
                    res = obj.as.arr->items[get_var(q->arg2).as.i];
                } else if (obj.type == VAL_DICT) {
                    res = dict_get(obj.as.dict, get_var(q->arg2).as.s);
                }
                set_var(q->res, res);
                break;
            }
            
            case OP_JMP: pc = atoi(q->res) - 1; break;
            case OP_JMP_F: if (get_var(q->arg1).as.i == 0) pc = atoi(q->res) - 1; break;
            
            case OP_PRINT_VAL: {
                Value v = get_var(q->arg1);
                if (v.type == VAL_INT) printf("%lld ", (long long)v.as.i);
                else if (v.type == VAL_STR) printf("%s ", v.as.s);
                else if (v.type == VAL_ARRAY) printf("[Array] ");
                else if (v.type == VAL_DICT) printf("{Dict} ");
                break;
            }
            case OP_PRINT_NL: printf("\n"); break;
            
            case OP_PARAM: param_stack[param_sp++] = get_var(q->arg1); break;
            case OP_CALL: {
                int p_count = atoi(q->arg2);
                Value f_val = get_var(q->arg1);
                char* f_name = f_val.type == VAL_STR ? f_val.as.s : q->arg1;
                
                Value target_pc = dict_get(func_map, f_name);
                if (target_pc.type != VAL_INT) { printf("Error: Func not found\n"); exit(1); }
                
                sp++;
                stack[sp].vars = new_dict();
                stack[sp].ret_pc = pc + 1;
                strcpy(stack[sp].ret_var, q->res);
                stack[sp].incoming_count = p_count;
                stack[sp].incoming_args = malloc(sizeof(Value) * p_count);
                for(int i = 0; i < p_count; i++) {
                    stack[sp].incoming_args[i] = param_stack[param_sp - p_count + i];
                }
                param_sp -= p_count;
                stack[sp].formal_idx = 0;
                
                pc = target_pc.as.i;
                continue;
            }
            case OP_FORMAL:
                dict_set(stack[sp].vars, q->arg1, stack[sp].incoming_args[stack[sp].formal_idx++]);
                break;
            case OP_RET_VAL: {
                Value ret_val = get_var(q->arg1);
                int ret_pc = stack[sp].ret_pc;
                char target_var[STR_LEN];
                strcpy(target_var, stack[sp].ret_var);
                free(stack[sp].incoming_args); // 簡易清理
                sp--;
                set_var(target_var, ret_val);
                pc = ret_pc;
                continue;
            }
            case OP_FUNC_END:
                if (sp > 0) {
                    int ret_pc = stack[sp].ret_pc;
                    char target_var[STR_LEN];
                    strcpy(target_var, stack[sp].ret_var);
                    free(stack[sp].incoming_args);
                    sp--;
                    set_var(target_var, make_nil());
                    pc = ret_pc;
                    continue;
                }
                break;
            default: break;
        }
        pc++;
    }
    printf("=== VM 執行完畢 ===\n");
}

int main() {
    // 這裡通常會呼叫 Parser 來填充 quads 陣列，
    // 為展示 VM 運作，這部分的實作可由編譯器前端驅動 vm_run()。
    printf("C 語言虛擬機已初始化準備就緒。\n");
    return 0;
}