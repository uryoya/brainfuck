#include <stdio.h>
#include <string.h>

#define MEM_SIZE 3000
#define BUF_SIZE 500
#define SCRIPT_SIZE 10000

int main(const int argc, const char **argv) {
    /* スクリプトファイルのロード */
    if(argc < 2) {
        printf("ファイルを指定してください.\n");
        return 1;
    }
    FILE *scriptfile;
    scriptfile = fopen(argv[1], "r");
    if(scriptfile == NULL) {
        printf("ファイルが開けません\n");
        return 1;
    }
    char script[SCRIPT_SIZE];
    char buf[BUF_SIZE];
    while(NULL != fgets(buf, BUF_SIZE, scriptfile)) {
        strncat(script, buf, BUF_SIZE);
    }
    fclose(scriptfile);

    /* スクリプトの実行 */
    int scriptlen = 0;
    int idx = 0;
    char memory[MEM_SIZE] = {0};
    int pointer = 0;
    scriptlen = strlen(script);
    while(idx < scriptlen) {
        switch(script[idx]) {
        case '>':
            ++pointer;
            break;
        case '<':
            --pointer;
            break;
        case '+':
            ++memory[pointer];
            break;
        case '-':
            --memory[pointer];
            break;
        case '.':
            printf("%c", memory[pointer]);
            break;
        case ',':
            // pass
            break;
        case '[':
            if(memory[pointer] == 0) {
                int roop = 1;
                while(roop > 0) {
                    ++idx;
                    if(script[idx] == ']') --roop;
                    else if(script[idx] == '[') ++roop;
                }
            }
            break;
        case ']':
            if(memory[pointer] != 0) {
                int roop = 1;
                while(roop > 0) {
                    --idx;
                    if(script[idx] == '[') --roop;
                    else if(script[idx] == ']') ++roop;
                }
            }
            break;
        default:
            break;
        }
        if (0 > pointer || MEM_SIZE <= pointer) {
            printf("ぬるぽ");
            return -1;
        }
        ++idx;
    }
    return 0;
}
