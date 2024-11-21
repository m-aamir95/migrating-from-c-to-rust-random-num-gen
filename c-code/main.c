#include "stdio.h"

int X = 123456789;

int seed_random_int(int seed){
    X = seed;
}

int get_random_int(void){
    X = X * 69069 + 362437;
    return X;
}


int main(void){

    for(int i=0; i < 10; i++){

        printf("%d\n", get_random_int());
    }

    return 0;
}