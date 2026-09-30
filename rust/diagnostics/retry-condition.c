#include <stdio.h>
#include <string.h>
int main(void) {
  int values[] = {0, 106}; int i;
  for (i=0;i<2;i++) {
    char work[60]; char *a;
    memset(work,values[i],sizeof work); strcpy(work,"objecti"); a=work+6;
    printf("tail=%d next=%d following=%d nul_member=%d retry=%d\n",values[i],a[1],a[2],strchr("aeiou",0)!=NULL,(*a=='i'&&a[2]&&strchr("aeiou",a[1]))!=0);
  }
  return 0;
}
