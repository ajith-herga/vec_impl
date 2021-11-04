#ifndef HASHSET_H
#define HASHSET_H
typedef enum {false, true} bool;
struct _myhashset;
typedef struct _myhashset MyHashSet;

MyHashSet* myHashSetCreate();
void myHashSetAdd(MyHashSet* obj, int key);
void myHashSetRemove(MyHashSet* obj, int key);
bool myHashSetContains(MyHashSet* obj, int key);
void myHashSetFree(MyHashSet* obj);
#endif /* HASHSET_H */
