#ifndef TRIEBUST_H
#define TRIEBUST_H
struct _mytrieburst;
typedef struct _mytrieburst MyTrieBurst;

MyTrieBurst* myTrieBurstCreate();
void myTrieBurstInsert(MyTrieBurst* obj, const char * word);
bool myTrieBurstSearch(MyTrieBurst* obj, const char * word);
bool myTrieBurstStartsWith(MyTrieBurst* obj, const char * prefix);
void myTrieBurstFree(MyTrieBurst* obj);

#endif /* TRIEBURST_H */
