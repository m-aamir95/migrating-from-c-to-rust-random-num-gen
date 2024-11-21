#include "stdio.h"
#include "vrandom.h"

unsigned long X = 123456789;

void seed_random_int(unsigned long seed){
    X = seed;
}

unsigned long get_random_int(void){
    X = X * 69069 + 362437;
    return X;
}


int main(void){

    for(int i=0; i < 10; i++){

        printf("%ld\n", get_random_int());
    }

    return 0;
}