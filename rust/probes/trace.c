#include <gkstring.h>
#include <stdio.h>
#include <stdlib.h>
static void h(FILE *f,const char*s,size_t n){size_t i; fputc('|',f);for(i=0;i<n && s[i];i++)fprintf(f,"%02x",(unsigned char)s[i]);}
static void form(FILE*f,word_form w){fprintf(f,"|%u",w.f_voice);fprintf(f,"|%u",w.f_mood);fprintf(f,"|%u",w.f_tense);fprintf(f,"|%u",w.f_person);fprintf(f,"|%u",w.f_number);fprintf(f,"|%u",w.f_case);fprintf(f,"|%u",w.f_degree);fprintf(f,"|%u",w.f_gender);}
#define META(v) do { form(f,(v)->gs_forminfo); fprintf(f,"|%lld",(long long)(v)->gs_steminfo);fprintf(f,"|%lld",(long long)(v)->gs_derivtype);fprintf(f,"|%lld",(long long)(v)->gs_dialect);fprintf(f,"|%lld",(long long)(v)->gs_geogregion); for(i=0;i<12;i++)fprintf(f,"|%u",(unsigned char)(v)->gs_morphflags[i]);h(f,(v)->st_domains,21); }while(0)
static void gs(FILE*f,const gk_string*v){int i;META(v);h(f,v->gs_gkstring,60);}
void morph_trace_word(const char*tag,const gk_word*v){FILE*f;const char*p=getenv("MORPH_TRACE_FILE");int i,j;if(!p)return;f=fopen(p,"a");if(!f)abort();fprintf(f,"%s",tag);if(!v){fputs("|null\n",f);fclose(f);return;}META(v);fprintf(f,"|%d|%d",v->gs_prntflags,v->gw_totanal);
h(f,v->st_lemma,60);
h(f,v->st_rawprvb,60);
h(f,v->st_rawword,60);
h(f,v->st_workword,60);
h(f,v->st_crasis,60);
gs(f,&v->gs_preverb);
gs(f,&v->gs_aug1);
gs(f,&v->gs_stem);
gs(f,&v->gs_suffix);
gs(f,&v->gs_endstring);
for(j=0;j<v->gw_totanal;j++){const gk_analysis*a=v->gw_analysis+j;fputs("|analysis",f);META(a);
h(f,a->st_lemma,60);
h(f,a->st_rawprvb,60);
h(f,a->st_rawword,60);
h(f,a->st_workword,60);
h(f,a->st_crasis,60);
h(f,a->st_dictform,60);
h(f,a->st_engform,60);
gs(f,&a->gs_preverb);
gs(f,&a->gs_aug1);
gs(f,&a->gs_stem);
gs(f,&a->gs_suffix);
gs(f,&a->gs_endstring);
}fputc('\n',f);fclose(f);}
