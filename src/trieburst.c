typedef enum {false, true} bool;
#include "trieburst.h"

#include <stdlib.h>
#include <string.h>

struct strings {
    struct strings* next;
    char* word;
};

struct node_header {
    bool is_branch;
    bool is_word;
    unsigned int level;
};

#define CHARS 26
struct node_branch {
    struct node_header header;
    struct node_header* chars[CHARS];
};

struct node_leaf {
    struct node_header header;
    unsigned int strings_count;
    struct strings* strs_head;
};

struct _mytrieburst {
    struct node_header *head;
};

static struct node_header *
get_node(struct node_branch* nb, char letter) {
    int index = (int)letter - (int)'a';
    return nb->chars[index];
}

static void
add_node(struct node_branch* nb, char letter, struct node_header *node) {
    int index = (int)letter - (int)'a';
    nb->chars[index] = node;
}

static struct node_branch*
get_init_node_branch(unsigned int level) {
    struct node_branch* nb = calloc(1, sizeof(struct node_branch));
    nb->header.is_branch = true;
    nb->header.level = level;
    return nb;
}

static struct node_branch*
add_node_branch(struct node_branch *parent, char letter) {
    struct node_branch* nb = get_init_node_branch(parent->header.level + 1);

    add_node(parent, letter, (struct node_header *)nb);
    return nb;
}

static struct node_leaf*
get_init_node_leaf(unsigned int level) {
    struct node_leaf* nl = calloc(1, sizeof(struct node_leaf));
    nl->header.level = level;
    return nl;
}

static struct node_leaf*
add_node_leaf(struct node_branch *parent, char letter) {
    struct node_leaf *nl = get_init_node_leaf(parent->header.level + 1);

    add_node(parent, letter, (struct node_header *)nl);
    return nl;
}

static void
end_word(struct node_header *node) {
    node->is_word = true;
}

static bool
node_is_word(struct node_header *node) {
    return node->is_word;
}

static bool
node_level(struct node_header *node) {
    return node->level;
}

#define BURST_LEAF_COUNT 10
#define MAX_SIZE 2000
typedef enum {valid, invalid} result;
static result
search_string_leaf(struct node_leaf *nl, const char *word, bool startswith, bool *found) {
    char *in_leaf_suffix;
    struct strings *nl_item;
    bool found_string = false;
    result res = valid;

    unsigned int leaf_level = node_level((struct node_header *)nl);

    if (word[leaf_level] == '\0' && node_is_word((struct node_header *)nl)) {
        found_string = true;
        goto out;
    }
    if (leaf_level >= MAX_SIZE) {
        res = invalid;
        goto out;
    }
    in_leaf_suffix = word + leaf_level;
    nl_item = nl->strs_head;
    while(nl_item) {
        char* store_leaf_suffix = nl_item->word + leaf_level;
        size_t len_cmp = MAX_SIZE - leaf_level;
        if (startswith)
            len_cmp = strlen(word) - leaf_level;
        if (strncmp(in_leaf_suffix, store_leaf_suffix, len_cmp) == 0) {
            found_string = true;
            goto out;
        }
        nl_item = nl_item->next;
    }
out:
    *found = found_string;
    return res;
}

static result
save_as_word_copy(struct node_leaf *nl, const char *word) {
    struct strings* item = malloc(sizeof(struct strings));

    item->word = strndup(word, MAX_SIZE);
    if (!item->word)
        return invalid;
    item->next = nl->strs_head;
    nl->strs_head = item;
    nl->strings_count++;
    return valid;
}

static result
save_as_word_move(struct node_leaf *nl, char *word) {
    struct strings* item = malloc(sizeof(struct strings));

    item->word = word;
    item->next = nl->strs_head;
    nl->strs_head = item;
    nl->strings_count++;
    return valid;
}

static bool
leaf_should_burst(struct node_leaf *nl) {
    if (node_level((struct node_header *)nl) > 2)
        return nl->strings_count > BURST_LEAF_COUNT;
    else
        return nl->strings_count > 1;
}

static result
burst_branch_from_leaf(struct node_leaf *nl, struct node_branch *nb) {
    struct strings *nl_item;
    unsigned int leaf_level = node_level((struct node_header *)nl);

    if (node_level((struct node_header *)nb) != leaf_level)
        return invalid;
    if (node_is_word((struct node_header *)nl))
        end_word((struct node_header *)nb);
    nl_item = nl->strs_head;
    while(nl_item) {
        struct node_header *nh;
        struct node_leaf *nl = NULL;
        char* store_leaf_suffix = nl_item->word + leaf_level;
        if (*store_leaf_suffix == '\0')
            return invalid;
        nh = get_node(nb, *store_leaf_suffix);
        if (nh == NULL) {
            nl = add_node_leaf(nb, *store_leaf_suffix);
            if (nl == NULL)
                return invalid;
        } else if (nh->is_branch) {
            return invalid;
        } else {
            nl = (struct node_leaf *)nh;
        }
        store_leaf_suffix++;
        if (*store_leaf_suffix == '\0') {
            end_word((struct node_header *)nl);
        } else {
            result res = save_as_word_move(nl, nl_item->word);
            if (res == invalid)
                return res;
            // Word moved.
            nl_item->word = NULL;
        }
        nl_item = nl_item->next;
    }
    return valid;
}

static void
node_leaf_free(struct node_leaf** nl)
{
    struct node_leaf *temp;
    struct strings *nl_item, *nl_item_temp;
    if (!nl || !*nl)
        return;

    temp = *nl;
    nl_item = temp->strs_head;
    while(nl_item) {
       free(nl_item->word);
       nl_item->word = NULL;
       nl_item_temp = nl_item;
       nl_item = nl_item->next;
       free(nl_item_temp);
    }
    free(temp);
    *nl = NULL;
}

static void
node_free(struct node_header** nh);

static void
node_branch_free(struct node_branch** nb) {
    struct node_branch *temp;
    if (!nb || !*nb)
        return;

    temp = *nb;
    for (int i = 0; i < CHARS; i++) {
        node_free(&temp->chars[i]);
    }
    free(temp);
    *nb = NULL;
}

void
node_free(struct node_header** nh) {
    if (!nh || !*nh)
        return;
    if ((*nh)->is_branch)
        node_branch_free((struct node_branch **)nh);
    else
        node_leaf_free((struct node_leaf **)nh);
}


MyTrieBurst* myTrieBurstCreate() {
    MyTrieBurst* trie = malloc(sizeof(MyTrieBurst));
    trie->head = (struct node_header *)get_init_node_leaf(0);
    return trie;
}
/*
 * Return pointer to last obj with offset in word.
 * Determine if more needs to be added based on level (offset)
 * If head is not null and offset_out is not changed, then the
 * input is invalid or none, both of which are not present in
 * the trie. Returns last, if found, then check its level against
 * offset of word. If word at its end and is_word is set,
 * then word is present. To insert. depends on if last is leaf or
 * branch. If branch, allocate leaf and add. If leaf, set string,
 * check if burst, replace with branch if so.
 */
static result
getLast(MyTrieBurst* obj, const char *word, struct node_header **nh_out, struct node_header **parent_out) {
    struct node_header *next, *temp = obj->head, *prev;
    unsigned int wi = 0;

    if (!word || !temp || word[wi] == '\0' || !nh_out)
        return invalid;
    prev = temp;
    while (word[wi] != '\0' && temp->is_branch) {
        struct node_branch *tempb = (struct node_branch *)temp;
        if (word[wi] < 'a' || word[wi] > 'z')
            return invalid;
        next = get_node(tempb, word[wi]);
        if (!next)
            break;
        prev = temp;
        temp = next;
        wi++;
    }

    if (!temp || wi != temp->level)
        return invalid;
    *nh_out = temp;
    if (parent_out)
        *parent_out = prev;
    return valid;
}

// Assumes size <= 2000, all small, [a,z].
void
myTrieBurstInsert(MyTrieBurst* obj, const char * word) {
   result res;
   struct node_header *nh, *parent_nh;
   struct node_branch *nb;
   struct node_leaf *nl;
   res = getLast(obj, word, &nh, &parent_nh);
   if (res == invalid || !nh || !parent_nh)
       return;
   if (word[nh->level] == '\0' && nh->is_word)
       return;
   if (nh->is_branch) {
       // Branch: allocate leaf, add and update nh.
       nb = (struct node_branch *)nh;
       nl = add_node_leaf(nb, word[nh->level]);
       if (word[nh->level] == '\0')
           end_word(nh);
       else
           save_as_word_copy(nl, word);
   } else {
       bool found = false;
       nl = (struct node_leaf *)nh;
       res = search_string_leaf(nl, word, false, &found);
       if (res == invalid || found == true)
           return;
       save_as_word_copy(nl, word);
       if (leaf_should_burst(nl)) {
           if (nh->level == 0) {
               if (nh != parent_nh || parent_nh != obj->head)
                   return;
               // Replace head with new branch
               nb = get_init_node_branch(0);
               obj->head = (struct node_header *)nb;
           } else {
               nb = add_node_branch((struct node_branch *)parent_nh, word[nh->level - 1]);
           }
           burst_branch_from_leaf(nl, nb);
           node_leaf_free(&nl);
       }
   }
}

static bool
myTrieBurstSearchOption(MyTrieBurst* obj, const char *word, bool startswith) {
   result res;
   struct node_header *nh, *parent_nh;
   struct node_leaf *nl;
   res = getLast(obj, word, &nh, &parent_nh);
   if (res == invalid || !nh || !parent_nh)
       return false;
   if (word[nh->level] == '\0' && (nh->is_word || startswith))
       return true;
   if (nh->is_branch) {
       return false;
   } else {
       bool found = false;
       nl = (struct node_leaf *)nh;
       res = search_string_leaf(nl, word, startswith, &found);
       return (res == valid && found == true);
   }
}

bool myTrieBurstSearch(MyTrieBurst* obj, const char * word) {
    return myTrieBurstSearchOption(obj, word, false);
}

bool myTrieBurstStartsWith(MyTrieBurst* obj, const char * prefix) {
    return myTrieBurstSearchOption(obj, prefix, true);
}

void myTrieBurstFree(MyTrieBurst* obj) {
    if (obj)
        node_free(&obj->head);
    free(obj);
}

/**
 * Your Trie struct will be instantiated and called as such
 *
 * Trie* obj = trieCreate();
 * trieInsert(obj, word);
 *
 * bool param_2 = trieSearch(obj, word);
 * bool param_3 = trieStartsWith(obj, prefix);
 *
 * trieFree(obj);
 *
 * All are lower chars from a to z. Length upto 2000, insert, search
 * and starts with upto 3 * 10^4 max.
*/

