#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

#define MAX_TOKEN 256
#define MAX_QUAD 4096
#define MAX_STACK 128
#define MAX_VARS 1024
#define MAX_STR_POOL 1024

/* =====================================================
   Value system
   ===================================================== */

typedef enum {
    VAL_INT,
    VAL_STR
} ValueType;

typedef struct {
    ValueType type;
    int i;
    char *s;
} Value;

Value make_int(int x){
    Value v; v.type=VAL_INT; v.i=x; v.s=NULL; return v;
}

Value make_str(char *s){
    Value v; v.type=VAL_STR; v.s=strdup(s); return v;
}

/* =====================================================
   Quad
   ===================================================== */

typedef struct {
    char op[32];
    char a1[64];
    char a2[64];
    char res[64];
} Quad;

Quad quads[MAX_QUAD];
int quad_count=0;

int emit(char *op,char *a1,char *a2,char *r){
    Quad *q=&quads[quad_count];
    strcpy(q->op,op);
    strcpy(q->a1,a1);
    strcpy(q->a2,a2);
    strcpy(q->res,r);
    printf("%03d: %-10s %-8s %-8s %-8s\n",
        quad_count,op,a1,a2,r);
    return quad_count++;
}

/* =====================================================
   Lexer
   ===================================================== */

typedef enum {
    TK_EOF,
    TK_ID,
    TK_NUM,
    TK_STRING,

    TK_LPAREN, TK_RPAREN,
    TK_LBRACE, TK_RBRACE,
    TK_PLUS, TK_MINUS,
    TK_MUL, TK_DIV,
    TK_ASSIGN,
    TK_SEMI,

    TK_IF,
    TK_WHILE,
    TK_PRINT
} TokenType;

typedef struct {
    TokenType type;
    char text[128];
} Token;

char *src;
int pos=0;
Token cur;

void next_token(){

    while(isspace(src[pos])) pos++;

    if(src[pos]==0){
        cur.type=TK_EOF;
        return;
    }

    if(isalpha(src[pos])){
        int start=pos;
        while(isalnum(src[pos])) pos++;

        strncpy(cur.text,src+start,pos-start);
        cur.text[pos-start]=0;

        if(strcmp(cur.text,"if")==0) cur.type=TK_IF;
        else if(strcmp(cur.text,"while")==0) cur.type=TK_WHILE;
        else if(strcmp(cur.text,"print")==0) cur.type=TK_PRINT;
        else cur.type=TK_ID;
        return;
    }

    if(isdigit(src[pos])){
        int start=pos;
        while(isdigit(src[pos])) pos++;

        strncpy(cur.text,src+start,pos-start);
        cur.text[pos-start]=0;

        cur.type=TK_NUM;
        return;
    }

    char c=src[pos++];

    switch(c){
        case '(': cur.type=TK_LPAREN; break;
        case ')': cur.type=TK_RPAREN; break;
        case '{': cur.type=TK_LBRACE; break;
        case '}': cur.type=TK_RBRACE; break;
        case '+': cur.type=TK_PLUS; break;
        case '-': cur.type=TK_MINUS; break;
        case '*': cur.type=TK_MUL; break;
        case '/': cur.type=TK_DIV; break;
        case '=': cur.type=TK_ASSIGN; break;
        case ';': cur.type=TK_SEMI; break;
        default:
            printf("unknown char %c\n",c);
            exit(1);
    }
}

/* =====================================================
   Parser
   ===================================================== */

int t_index=0;

char* new_temp(){
    static char buf[32];
    sprintf(buf,"t%d",++t_index);
    return strdup(buf);
}

char* expression();

char* factor(){

    if(cur.type==TK_NUM){
        char *t=new_temp();
        emit("IMM",cur.text,"-",t);
        next_token();
        return t;
    }

    if(cur.type==TK_ID){
        char *name=strdup(cur.text);
        next_token();
        return name;
    }

    if(cur.type==TK_LPAREN){
        next_token();
        char *e=expression();
        if(cur.type!=TK_RPAREN){
            printf(") expected\n");
            exit(1);
        }
        next_token();
        return e;
    }

    printf("factor error\n");
    exit(1);
}

char* term(){

    char *l=factor();

    while(cur.type==TK_MUL || cur.type==TK_DIV){

        TokenType op=cur.type;
        next_token();

        char *r=factor();

        char *t=new_temp();

        if(op==TK_MUL) emit("MUL",l,r,t);
        else emit("DIV",l,r,t);

        l=t;
    }

    return l;
}

char* expression(){

    char *l=term();

    while(cur.type==TK_PLUS || cur.type==TK_MINUS){

        TokenType op=cur.type;
        next_token();

        char *r=term();

        char *t=new_temp();

        if(op==TK_PLUS) emit("ADD",l,r,t);
        else emit("SUB",l,r,t);

        l=t;
    }

    return l;
}

void statement(){

    if(cur.type==TK_PRINT){

        next_token();

        if(cur.type!=TK_LPAREN){
            printf("( expected\n");
            exit(1);
        }

        next_token();

        char *v=expression();

        emit("PRINT",v,"-","-");

        if(cur.type!=TK_RPAREN){
            printf(") expected\n");
            exit(1);
        }

        next_token();

        if(cur.type!=TK_SEMI){
            printf("; expected\n");
            exit(1);
        }

        next_token();
        return;
    }

    if(cur.type==TK_ID){

        char name[64];
        strcpy(name,cur.text);
        next_token();

        if(cur.type!=TK_ASSIGN){
            printf("= expected\n");
            exit(1);
        }

        next_token();

        char *v=expression();

        emit("STORE",v,"-",name);

        if(cur.type!=TK_SEMI){
            printf("; expected\n");
            exit(1);
        }

        next_token();
        return;
    }

    printf("unknown statement\n");
    exit(1);
}

void parse(){

    next_token();

    while(cur.type!=TK_EOF)
        statement();
}

/* =====================================================
   VM
   ===================================================== */

typedef struct {
    char name[64];
    Value val;
} Var;

Var vars[MAX_VARS];
int var_count=0;

Value get_var(char *name){

    if(isdigit(name[0]))
        return make_int(atoi(name));

    for(int i=0;i<var_count;i++)
        if(strcmp(vars[i].name,name)==0)
            return vars[i].val;

    return make_int(0);
}

void set_var(char *name,Value v){

    for(int i=0;i<var_count;i++)
        if(strcmp(vars[i].name,name)==0){
            vars[i].val=v;
            return;
        }

    strcpy(vars[var_count].name,name);
    vars[var_count].val=v;
    var_count++;
}

void run_vm(){

    printf("\n=== VM RUN ===\n");

    for(int pc=0;pc<quad_count;pc++){

        Quad *q=&quads[pc];

        if(strcmp(q->op,"IMM")==0){
            set_var(q->res,make_int(atoi(q->a1)));
        }

        else if(strcmp(q->op,"ADD")==0){
            Value a=get_var(q->a1);
            Value b=get_var(q->a2);
            set_var(q->res,make_int(a.i+b.i));
        }

        else if(strcmp(q->op,"SUB")==0){
            Value a=get_var(q->a1);
            Value b=get_var(q->a2);
            set_var(q->res,make_int(a.i-b.i));
        }

        else if(strcmp(q->op,"MUL")==0){
            Value a=get_var(q->a1);
            Value b=get_var(q->a2);
            set_var(q->res,make_int(a.i*b.i));
        }

        else if(strcmp(q->op,"DIV")==0){
            Value a=get_var(q->a1);
            Value b=get_var(q->a2);
            set_var(q->res,make_int(a.i/b.i));
        }

        else if(strcmp(q->op,"STORE")==0){
            set_var(q->res,get_var(q->a1));
        }

        else if(strcmp(q->op,"PRINT")==0){
            Value v=get_var(q->a1);
            printf("[OUTPUT] %d\n",v.i);
        }
    }
}

/* =====================================================
   Main
   ===================================================== */

char* load_file(char *path){

    FILE *f=fopen(path,"rb");

    if(!f){
        printf("cannot open %s\n",path);
        exit(1);
    }

    fseek(f,0,SEEK_END);
    long size=ftell(f);
    rewind(f);

    char *buf=malloc(size+1);
    fread(buf,1,size,f);
    buf[size]=0;

    fclose(f);
    return buf;
}

int main(int argc,char **argv){

    if(argc<2){
        printf("usage: compiler source.p0\n");
        return 0;
    }

    src=load_file(argv[1]);

    printf("=== QUAD ===\n");

    parse();

    run_vm();

    return 0;
}