// #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
use core::arch::x86_64::{
    _mm256_testz_si256, _mm_add_epi32, _mm_set1_epi32, _mm_store_epi32, _CMP_NLT_UQ,
};

// #[cfg(target_feature = "avx2")]
// #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
use core::arch::x86_64::{_mm256_cmp_pd, _mm256_loadu_pd, _mm256_min_pd, _mm256_storeu_pd};
use std::arch::x86_64::{
    __m256i, _mm256_add_epi32, _mm256_blendv_ps, _mm256_castpd256_pd128, _mm256_cmp_ps,
    _mm256_loadu_ps, _mm256_min_ps, _mm256_permutevar8x32_epi32, _mm256_set1_epi32,
    _mm256_set_epi32, _mm256_store_epi32, _mm256_storeu_ps, _mm_blendv_ps, _mm_set_epi32,
    _CMP_LT_OQ,
};
use std::ptr;

use crate::types::Content;

// #[cfg(target_feature = "avx2")]
// #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub(crate) unsafe fn get_best_from_dists_f32_avx2<T: Content, const B: usize>(
    acc: &[f32; B],
    items: &[T; B],
    best_dist: &mut f32,
    best_item: &mut T,
) {
    let mut index_v = _mm256_set_epi32(0, 1, 2, 3, 4, 5, 6, 7);
    let mut min_dist_indexes_v = _mm256_set1_epi32(-1);
    let all_eights = _mm256_set1_epi32(8);
    let mut min_dists = [*best_dist; 8];
    let mut min_dists_v = _mm256_loadu_ps(ptr::addr_of!(min_dists[0]));

    let mut any_is_better = false;
    for chunk in acc.as_chunks_unchecked::<8>().iter() {
        let chunk_v = _mm256_loadu_ps(ptr::addr_of!(chunk[0]));

        let is_better = _mm256_cmp_ps(chunk_v, min_dists_v, _CMP_LT_OQ);

        let these_better = _mm256_testz_si256(
            std::mem::transmute(is_better),
            std::mem::transmute(is_better),
        );

        any_is_better |= these_better == 0;

        min_dists_v = _mm256_min_ps(min_dists_v, chunk_v);

        min_dist_indexes_v = std::mem::transmute(_mm256_blendv_ps(
            std::mem::transmute(min_dist_indexes_v),
            std::mem::transmute(index_v),
            is_better,
        ));

        index_v = _mm256_add_epi32(index_v, all_eights);
    }

    if !any_is_better {
        return;
    }

    let mut min_dist_indexes = [0i32; 8];
    _mm256_store_epi32(ptr::addr_of_mut!(min_dist_indexes[0]), min_dist_indexes_v);
    _mm256_storeu_ps(ptr::addr_of_mut!(min_dists[0]), min_dists_v);

    for (i, dist) in min_dists.iter().enumerate() {
        if *dist < *best_dist {
            *best_dist = *dist;
            *best_item = items[min_dist_indexes[i] as usize];
        }
    }
}