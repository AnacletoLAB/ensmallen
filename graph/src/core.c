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
            ptr2 += !is_less;
            ptr1++;
            ptrt++;
        } else {    
            if ((ptr1 - end1) >= 4) {
                __m256i v2s   = _mm256_lddqu_si256((__m256i *) ptr1);
                __m256i broad = _mm256_broadcastd_epi32(_mm_set1_epi32(v1));
                __m256i cmp   = _mm256_cmpgt_epi32(v2s, broad);
                int mask = _mm256_movemask_epi8(cmp);
                ptr2 += _lzcnt_u64(~mask) >> 2;
            } else {
                ptr2 += 1;
            }
        }
    }
    float one = 1.0;
    float coef = explore_weight;
    __m256 ones = _mm256_broadcast_ss(&one);
    __m256 default_coeffs = _mm256_broadcast_ss(&coef);

    __m256i srcs =  _mm256_castps_si256(_mm256_broadcast_ss((float*) &src));
    __m256i dsts =  _mm256_castps_si256(_mm256_broadcast_ss((float*) &dst));

    while ((ptr1 - end1) >= 8) {
        // v2s = *ptr1
        __m256i v2s = _mm256_lddqu_si256((__m256i *)ptr1);
        
        // mask = (v2s == src) || (v2s == dst)
        __m256 mask = _mm256_castsi256_ps(_mm256_or_si256(
            _mm256_cmpeq_epi32(v2s, srcs),
            _mm256_cmpeq_epi32(v2s, dsts)
        ));

        // mask & default_coeffs | (!mask & ones)
        __m256 coeffs = _mm256_or_ps(
            _mm256_and_ps(mask, default_coeffs),
            _mm256_andnot_ps(mask, ones)
        );

        // ptrt *= coeffs
        __m256 trans = _mm256_loadu_ps(ptrt);
        _mm256_storeu_ps(
            ptrt,
            _mm256_mul_ps(
                trans,
                coeffs
            )
        );
        ptrt += 8;
    }
    while(ptr1 < end1) {    
        v1 = *ptr1++;
        *ptrt++  *= 1.0 + (v1 != src && v1 != dst) * (explore_weight - 1.0);
    }
}

extern void c_update_return_explore_weight_transition(
    f32 *transition,
    u32 *destinations,
    u32 destinations_len,
    u32 *previous_destinations,
    u32 previous_destinations_len,
    f32 explore_weight,
    f32 return_weight,
    u32 src,
    u32 dst
) {
    u32 v1, v2, *ptr1 = destinations, *ptr2 = previous_destinations;
    f32 *ptrt = transition;
    u32 *end1 = &destinations[destinations_len];
    u32 *end2 = &previous_destinations[previous_destinations_len];

    while(ptr1 < end1 && ptr2 < end2) {
        v1 = *ptr1; v2 = *ptr2;
        if (v1 == src || v1 == dst) {
            *ptrt *= return_weight;
            ptr1++;
            continue;
        }
        if(v1 <= v2) {
            int is_less = v1 < v2;
            if(is_less && v1 != src && v1 != dst){
                *ptrt *= explore_weight;
            }
            ptr2 += !is_less;
            ptr1++;
            ptrt++;
        } else {
            ptr2++;
        }
    }

    while(ptr1 < end1) {
        v1 = *ptr1++;
        int cond = (v1 != src && v1 != dst);
        *ptrt++  *= (
            1.0 + cond * (explore_weight - 1.0)
        ) * (
            1.0 + !cond * (return_weight - 1.0)
        );
    }
}