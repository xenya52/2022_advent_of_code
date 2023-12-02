#include <stdio.h>
#include <stdlib.h>

#define ASCII_DOWGRADE 48
#define MAX_LINE_LENGTH 100

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
    
    char *cur_line;
    char index;
    char temp_first = 0;
    char temp_last = 0;
    int sum = 0;
    int is_first_in = 0; //bool

    while (index != EOF) {
        index = fgetc(f_ptr);
        if(index == '\n') {
            printf("%s\n",cur_line);
        }
    }
/*
    if(f_ptr != NULL) {
        while (index != EOF) {
            index = fgetc(f_ptr);
            
            if(index == '\n') {
                sum += (temp_first - ASCIIDOWGRADE) * 10 + temp_last - ASCIIDOWGRADE;
                is_first_in = 0;
            }
            else if(index <= '9' && index >= '0' && is_first_in == 1) {
                temp_last = index;
            }
            else if(index <= '9' && index >= '0' && is_first_in != 1) {
                temp_first = index;
                temp_last = index;
                is_first_in = 1;
            }
        }    
    }
    else {
        printf("Error: No file / Not able to open");
    }
*/
    printf("%d",sum);

    fclose(f_ptr);
}

/*
temp_first = every first digit
temp_last = every last digit
---IN ONE LINE!!!---
sum += "temp_first" + "temp_last"
*/