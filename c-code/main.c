#include "stdio.h"
#include "vrandom.h"


int main(void){

    for(int i=0; i < 10; i++){

        printf("%ld\n", get_random_int());
    }

    return 0;
}