

use std::arch::x86_64::{
    __m256i,
    _mm256_lddqu_si256,
    _mm256_loadu_ps,
    _mm256_storeu_ps,
    _mm256_castps_si256,
    _mm256_castsi256_ps,
    _mm256_broadcastd_epi32,
    _mm256_broadcast_ss,
    _mm256_cmpgt_epi32,
    _mm256_cmpeq_epi32,
    _mm256_movemask_epi8,
    _mm_set1_epi32,
    _mm256_and_ps,
    _mm256_andnot_ps,
    _mm256_or_si256,
    _mm256_or_ps,
    _mm256_mul_ps,
};

pub(crate) unsafe fn c_update_explore_weight_transition_2(
    transitions: &mut [f32],
    destinations: &mut [u32],
    previous_destinations: &[u32],
    explore_weight: f32,
    src: u32,
    dst: u32,
) {
    debug_assert!(transitions.len() == destinations.len());
    let mut ptr1 = destinations.as_ptr();
    let end1 = ptr1.add(destinations.len());
    let mut ptr2 = previous_destinations.as_ptr();
    let end2 = ptr2.add(destinations.len());
    let mut ptrt = transitions.as_mut_ptr();


    while ptr1 < end1 && ptr2 < end2 {
        let v1 = *ptr1;    
        let v2 = *ptr2;    
        if v1 <= v2 {
            let is_less = v1 < v2;
            if is_less && v1 != src && v1 != dst {
                *ptrt *= explore_weight;
            }
            ptr2 = ptr2.add(is_less as usize as _);
            ptr1 = ptr1.add(1);
            ptrt = ptrt.add(1);
        } else {
            if end1.sub(ptr1 as usize) as usize >= 4 {
                let v2s = _mm256_lddqu_si256(ptr1 as _);
                let broad = _mm256_broadcastd_epi32(_mm_set1_epi32(v1 as _));
                let cmp = _mm256_cmpgt_epi32(v2s, broad);
                let mask =  _mm256_movemask_epi8(cmp);
                ptr2 = ptr2.add((!mask).leading_zeros() as _);
            } else {
                ptr2 = ptr2.offset(1);
            }
        }
    }

    let ones = _mm256_broadcast_ss(&1.0);
    let default_coeffs = _mm256_broadcast_ss(&explore_weight);

    let srcs = _mm256_castps_si256(_mm256_broadcast_ss(&(src as f32)));
    let dsts = _mm256_castps_si256(_mm256_broadcast_ss(&(dst as f32)));

    while end1.sub(ptr1 as usize) as usize >= 8 {
        // v2s = *ptr1
        let v2s = _mm256_lddqu_si256(ptr1 as _);
        // mask = (v2s == src) || (v2s == dst)
        let mask = _mm256_castsi256_ps(_mm256_or_si256(
            _mm256_cmpeq_epi32(v2s, srcs),
            _mm256_cmpeq_epi32(v2s, dsts)
        ));
        // mask & default_coeffs | (!mask & ones)
        let coeffs = _mm256_or_ps(
            _mm256_and_ps(mask, default_coeffs),
            _mm256_andnot_ps(mask, ones)
        );

        // ptrt *= coeffs
        let trans = _mm256_loadu_ps(ptrt);
        _mm256_storeu_ps(
            ptrt,
            _mm256_mul_ps(
                trans,
                coeffs
            )
        );
        ptrt = ptrt.add(8);
    }

    while ptr1 < end1 {    
        let v1 = *ptr1;
        ptr1 = ptr1.add(1);
        *ptrt  *= 1.0 
            + ((v1 != src && v1 != dst) as usize as f32) 
            * ((explore_weight - 1.0) as usize as f32);
        ptrt = ptrt.add(1);
    }
}