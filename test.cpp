#include <cstdint>
#include <cstring>
#include <fstream>
#include <iostream>

#include "VMachine.h"
#include "VMachine__Syms.h"
#include <verilated.h>

static VMachine *top = new VMachine;

int main(int argc, char **argv) {
    
    scanf("%d%d",&top->a,&top->b);

    top->eval();

    printf("%d\n",top->out);

    delete top;
    return 0;
}