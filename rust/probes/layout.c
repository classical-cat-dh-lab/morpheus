#include <stddef.h>
#include <stdio.h>
#include <gkstring.h>
int main(void) {
 printf("{\"word_form\":%zu,\"word_form_align\":%zu,\"gk_string\":%zu,\"gk_word\":%zu,\"gk_analysis\":%zu,\"workword_offset\":%zu,\"analysis_offset\":%zu}\n", sizeof(word_form),_Alignof(word_form),sizeof(gk_string),sizeof(gk_word),sizeof(gk_analysis),offsetof(gk_word,st_workword),offsetof(gk_word,gw_analysis));
 return 0;
}
