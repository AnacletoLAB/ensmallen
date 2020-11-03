#include <x86intrin.h>
#include <stdio.h>

typedef float                 f32;
typedef double                f64;
typedef signed char           s8;
typedef char                  u8;
typedef short                 s16;
typedef unsigned short        u16;
typedef int                   s32;
typedef unsigned int          u32;
typedef long long             s64;
typedef unsigned long long    u64;

extern void c_update_explore_weight_transition(
    f32 *transition,
    u32 *destinations,
    u32 destinations_len,
    u32 *previous_destinations,
    u32 previous_destinations_len,
    f32 explore_weight,
    u32 src,
    u32 dst
) {
    u32 v1, v2, *ptr1 = destinations, *ptr2 = previous_destinations;
    f32 *ptrt = transition;
    u32 *end1 = &destinations[destinations_len];
    u32 *end2 = &previous_destinations[previous_destinations_len];

    while(ptr1 < end1 && ptr2 < end2) {
        v1 = *ptr1; v2 = *ptr2;
        if(v1 <= v2) {
            int is_less = v1 < v2;
            if(is_less && v1 != src && v1 != dst){
                *ptrt *= explore_weight;
            }
            ptr2 += is_less ? 0 : 1;
            ptr1++;
            ptrt++;
        } else {
            ptr2++;
        }
    }

    while(ptr1 < end1) {
        v1 = *ptr1++;
        *ptrt++  *= 1.0 + (v1 != src && v1 != dst) * (explore_weight - 1.0);
    }
}