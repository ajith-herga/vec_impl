#include "hashset.h"

#include <stdlib.h>

struct _myhashset {
    int32_t a[10000];
};

/** Initialize your data structure here. */

MyHashSet* myHashSetCreate() {
    MyHashSet* set = malloc(sizeof(MyHashSet));
    for (int i = 0; i < 10000; i++)
        set->a[i] = -1;
    return set;
}


int myHashSetPosition(MyHashSet* obj, int key, bool *present) {
    int pos = -1; 
    *present = false;
    if (key > 1000000 || key < 0)
        goto out;
    if (obj == NULL)
        goto out;
    int index = key % 10000;
    for (int probe = 0; probe < 10; probe++) {
        int peek = (index + probe) % 10000;
        int val = obj->a[peek];
        if (val == key) {
            pos = peek;
            *present = true;
            goto out;
        } else if (val == -2) {
            /* Note the first deleted space. */
            if (pos == -1) pos = peek;
        } else if (val == -1) {
            /* Stop at first empty, key absent.. */
            if (pos == -1) pos = peek;
            goto out;
        }
    }
out:
    return pos;
}
    
void myHashSetAdd(MyHashSet* obj, int key) {
    bool present;
    int pos = myHashSetPosition(obj, key, &present);
    /* Will silently fail if probe is above 10. */
    if (present || pos == -1)
        goto out;
    obj->a[pos] = key;
out:
    return;
}

void myHashSetRemove(MyHashSet* obj, int key) {
    bool present;
    int pos = myHashSetPosition(obj, key, &present);
    /* Should assert that not present means pos is -1. */
    if (!present || pos == -1)
        goto out;
    obj->a[pos] = -2;
out:
    return;
}

/** Returns true if this set contains the specified element */
bool myHashSetContains(MyHashSet* obj, int key) {
    bool present;
    myHashSetPosition(obj, key, &present);
    return present;  
}

void myHashSetFree(MyHashSet* obj) {
    free(obj);
}

/**
 * Your MyHashSet struct will be instantiated and called as such:
 * MyHashSet* obj = myHashSetCreate();
 * myHashSetAdd(obj, key);
 
 * myHashSetRemove(obj, key);
 
 * bool param_3 = myHashSetContains(obj, key);
 
 * myHashSetFree(obj);
*/
