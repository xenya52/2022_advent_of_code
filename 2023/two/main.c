#include <stdio.h>

int main(int argc, char *argv[]) {
    if(!argv[1]) {
        printf("You need to give the first parameter as the file name");
        return -1;
    }

    FILE *f_ptr = fopen(argv[1], "r");
    if(f_ptr == NULL) {
        printf("You given files is invalid");
        return -1;
    }
    printf("Hello mum");
}