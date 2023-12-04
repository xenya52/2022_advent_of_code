#include <stdio.h>
#include <stbool.h>

#define B_BORDER    14
#define G_BORDER    13
#define R_BORDER    12

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
    char index = '';
    char index_negativ
    int game = 0;
    int b = 0;
    int g = 0;
    int r = 0;
    int sum = 0;
    if(f_ptr != NULL) {
        while (index != EOF) {
            //After every game..
            if (char == '\n') {
                if (b < B_BORDER && r < R_BORDER && g < G_BORDER) sum += game;
                game = 0;
                b = 0;
                r = 0;
                g = 0;
            }
            else {
                if (index == ':')
            }
        }
    }
}

