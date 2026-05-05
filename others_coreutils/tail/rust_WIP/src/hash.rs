











use std::sync::atomic::{AtomicPtr, Ordering};

use std::option::Option;

use std::str;

use std::boxed::Box;
use std::ptr;
use std::process;
use std::mem;

use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn abort() -> !;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hash_tuning {
    pub shrink_threshold: libc::c_float,
    pub shrink_factor: libc::c_float,
    pub growth_threshold: libc::c_float,
    pub growth_factor: libc::c_float,
    pub is_n_buckets: bool,
}
pub type Hash_tuning = hash_tuning;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hash_table {
    pub bucket: *mut hash_entry,
    pub bucket_limit: *const hash_entry,
    pub n_buckets: size_t,
    pub n_buckets_used: size_t,
    pub n_entries: size_t,
    pub tuning: *const Hash_tuning,
    pub hasher: Hash_hasher,
    pub comparator: Hash_comparator,
    pub data_freer: Hash_data_freer,
    pub free_entry_list: *mut hash_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hash_entry {
    pub data: *mut libc::c_void,
    pub next: *mut hash_entry,
}
pub type Hash_data_freer = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type Hash_comparator = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> bool,
>;
pub type Hash_hasher = Option::<
    unsafe extern "C" fn(*const libc::c_void, size_t) -> size_t,
>;
pub type Hash_table = hash_table;
pub type Hash_processor = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> bool,
>;
#[inline]
fn rotr_sz(x: u64, n: i32) -> u64 {
    let size_bits = (std::mem::size_of::<u64>() * 8) as u32;
    let n = n as u32 % size_bits; // Ensure n is within the bounds of size_bits
    (x >> n) | (x << (size_bits - n))
}

static mut default_tuning: Hash_tuning = {
    let mut init = hash_tuning {
        shrink_threshold: 0.0f32,
        shrink_factor: 1.0f32,
        growth_threshold: 0.8f32,
        growth_factor: 1.414f32,
        is_n_buckets: 0 as libc::c_int != 0,
    };
    init
};
#[no_mangle]
pub fn hash_get_n_buckets(table: &Hash_table) -> usize {
    table.n_buckets as usize
}

#[no_mangle]
use std::convert::TryInto;

pub fn hash_get_n_buckets_used(table: &Hash_table) -> usize {
    table.n_buckets_used.try_into().unwrap()
}

#[no_mangle]
pub unsafe extern "C" fn hash_get_n_entries(mut table: *const Hash_table) -> size_t {
    return (*table).n_entries;
}
#[no_mangle]
pub fn hash_get_max_bucket_length(table: &Hash_table) -> usize {
    let mut max_bucket_length: usize = 0;
    let bucket_limit = table.bucket_limit as usize;
    let buckets = unsafe { std::slice::from_raw_parts(table.bucket, bucket_limit) };
    
    for bucket in buckets {
        if !bucket.data.is_null() {
            let mut cursor: *const hash_entry = bucket;
            let mut bucket_length: usize = 1;
            loop {
                cursor = unsafe { (*cursor).next };
                if cursor.is_null() {
                    break;
                }
                bucket_length += 1;
            }
            if bucket_length > max_bucket_length {
                max_bucket_length = bucket_length;
            }
        }
    }
    max_bucket_length
}

#[no_mangle]
pub unsafe extern "C" fn hash_table_ok(mut table: *const Hash_table) -> bool {
    let mut bucket: *const hash_entry = 0 as *const hash_entry;
    let mut n_buckets_used: size_t = 0 as libc::c_int as size_t;
    let mut n_entries: size_t = 0 as libc::c_int as size_t;
    bucket = (*table).bucket;
    while bucket < (*table).bucket_limit {
        if !((*bucket).data).is_null() {
            let mut cursor: *const hash_entry = bucket;
            n_buckets_used = n_buckets_used.wrapping_add(1);
            n_buckets_used;
            n_entries = n_entries.wrapping_add(1);
            n_entries;
            loop {
                cursor = (*cursor).next;
                if cursor.is_null() {
                    break;
                }
                n_entries = n_entries.wrapping_add(1);
                n_entries;
            }
        }
        bucket = bucket.offset(1);
        bucket;
    }
    if n_buckets_used == (*table).n_buckets_used && n_entries == (*table).n_entries {
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
#[no_mangle]
pub fn hash_print_statistics(
    table: &Hash_table,
    stream: &mut dyn std::io::Write,
) {
    let n_entries: usize = unsafe { hash_get_n_entries(table) }.try_into().unwrap();
    let n_buckets: usize = unsafe { hash_get_n_buckets(table) }.try_into().unwrap();
    let n_buckets_used: usize = unsafe { hash_get_n_buckets_used(table) }.try_into().unwrap();
    let max_bucket_length: usize = unsafe { hash_get_max_bucket_length(table) }.try_into().unwrap();
    
    writeln!(stream, "# entries:         {}", n_entries).unwrap();
    writeln!(stream, "# buckets:         {}", n_buckets).unwrap();
    writeln!(stream, "# buckets used:    {} ( {:.2}%)", n_buckets_used, 100.0 * n_buckets_used as f64 / n_buckets as f64).unwrap();
    writeln!(stream, "max bucket length: {}", max_bucket_length).unwrap();
}

unsafe extern "C" fn safe_hasher(
    mut table: *const Hash_table,
    mut key: *const libc::c_void,
) -> *mut hash_entry {
    let mut n: size_t = ((*table).hasher)
        .expect("non-null function pointer")(key, (*table).n_buckets);
    if !(n < (*table).n_buckets) {
        abort();
    }
    return ((*table).bucket).offset(n as isize);
}
#[no_mangle]
pub unsafe extern "C" fn hash_lookup(
    mut table: *const Hash_table,
    mut entry: *const libc::c_void,
) -> *mut libc::c_void {
    let mut bucket: *const hash_entry = safe_hasher(table, entry);
    let mut cursor: *const hash_entry = 0 as *const hash_entry;
    if ((*bucket).data).is_null() {
        return 0 as *mut libc::c_void;
    }
    cursor = bucket;
    while !cursor.is_null() {
        if entry == (*cursor).data as *const libc::c_void
            || ((*table).comparator)
                .expect("non-null function pointer")(entry, (*cursor).data)
                as libc::c_int != 0
        {
            return (*cursor).data;
        }
        cursor = (*cursor).next;
    }
    return 0 as *mut libc::c_void;
}
#[no_mangle]
pub fn hash_get_first(table: &Hash_table) -> Option<&libc::c_void> {
    if table.n_entries == 0 {
        return None;
    }
    
    let mut bucket = table.bucket as *const hash_entry;
    while bucket < table.bucket_limit as *const hash_entry {
        if let Some(data) = unsafe { (*bucket).data.as_ref() } {
            return Some(data);
        }
        unsafe {
            bucket = bucket.add(1);
        }
    }
    
    None
}

#[no_mangle]
pub unsafe extern "C" fn hash_get_next(
    mut table: *const Hash_table,
    mut entry: *const libc::c_void,
) -> *mut libc::c_void {
    let mut bucket: *const hash_entry = safe_hasher(table, entry);
    let mut cursor: *const hash_entry = 0 as *const hash_entry;
    cursor = bucket;
    loop {
        if (*cursor).data == entry as *mut libc::c_void && !((*cursor).next).is_null() {
            return (*(*cursor).next).data;
        }
        cursor = (*cursor).next;
        if cursor.is_null() {
            break;
        }
    }
    loop {
        bucket = bucket.offset(1);
        if !(bucket < (*table).bucket_limit) {
            break;
        }
        if !((*bucket).data).is_null() {
            return (*bucket).data;
        }
    }
    return 0 as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn hash_get_entries(
    mut table: *const Hash_table,
    mut buffer: *mut *mut libc::c_void,
    mut buffer_size: size_t,
) -> size_t {
    let mut counter: size_t = 0 as libc::c_int as size_t;
    let mut bucket: *const hash_entry = 0 as *const hash_entry;
    let mut cursor: *const hash_entry = 0 as *const hash_entry;
    bucket = (*table).bucket;
    while bucket < (*table).bucket_limit {
        if !((*bucket).data).is_null() {
            cursor = bucket;
            while !cursor.is_null() {
                if counter >= buffer_size {
                    return counter;
                }
                let fresh0 = counter;
                counter = counter.wrapping_add(1);
                let ref mut fresh1 = *buffer.offset(fresh0 as isize);
                *fresh1 = (*cursor).data;
                cursor = (*cursor).next;
            }
        }
        bucket = bucket.offset(1);
        bucket;
    }
    return counter;
}
#[no_mangle]
pub unsafe extern "C" fn hash_do_for_each(
    mut table: *const Hash_table,
    mut processor: Hash_processor,
    mut processor_data: *mut libc::c_void,
) -> size_t {
    let mut counter: size_t = 0 as libc::c_int as size_t;
    let mut bucket: *const hash_entry = 0 as *const hash_entry;
    let mut cursor: *const hash_entry = 0 as *const hash_entry;
    bucket = (*table).bucket;
    while bucket < (*table).bucket_limit {
        if !((*bucket).data).is_null() {
            cursor = bucket;
            while !cursor.is_null() {
                if !processor
                    .expect("non-null function pointer")((*cursor).data, processor_data)
                {
                    return counter;
                }
                counter = counter.wrapping_add(1);
                counter;
                cursor = (*cursor).next;
            }
        }
        bucket = bucket.offset(1);
        bucket;
    }
    return counter;
}
#[no_mangle]
pub fn hash_string(string: &str, n_buckets: usize) -> usize {
    let mut value: usize = 0;
    for ch in string.chars() {
        value = value
            .wrapping_mul(31)
            .wrapping_add(ch as usize)
            .wrapping_rem(n_buckets);
    }
    value
}

fn is_prime(mut candidate: u64) -> bool {
    if candidate < 2 {
        return false;
    }
    if candidate == 2 {
        return true;
    }
    if candidate % 2 == 0 {
        return false;
    }

    let mut divisor: u64 = 3;
    while divisor * divisor <= candidate {
        if candidate % divisor == 0 {
            return false;
        }
        divisor += 2; // Only check odd divisors
    }
    true
}

fn next_prime(mut candidate: u64) -> u64 {
    if candidate < 10 {
        candidate = 10;
    }
    candidate |= 1; // Ensure candidate is odd
    while candidate != u64::MAX && !unsafe { is_prime(candidate) } {
        candidate += 2; // Increment by 2 to check the next odd number
    }
    candidate
}

#[no_mangle]
pub fn hash_reset_tuning(tuning: &mut Hash_tuning) {
    *tuning = unsafe { default_tuning };
}

unsafe extern "C" fn raw_hasher(mut data: *const libc::c_void, mut n: size_t) -> size_t {
    let val: u64 = rotr_sz(data as u64, 3);
    return val.wrapping_rem(n);
}
unsafe extern "C" fn raw_comparator(
    a: *const libc::c_void,
    b: *const libc::c_void,
) -> bool {
    a == b
}

unsafe extern "C" fn check_tuning(mut table: *mut Hash_table) -> bool {
    let mut tuning: *const Hash_tuning = (*table).tuning;
    let mut epsilon: libc::c_float = 0.;
    if tuning == &default_tuning as *const Hash_tuning {
        return 1 as libc::c_int != 0;
    }
    epsilon = 0.1f32;
    if epsilon < (*tuning).growth_threshold
        && (*tuning).growth_threshold < 1 as libc::c_int as libc::c_float - epsilon
        && 1 as libc::c_int as libc::c_float + epsilon < (*tuning).growth_factor
        && 0 as libc::c_int as libc::c_float <= (*tuning).shrink_threshold
        && (*tuning).shrink_threshold + epsilon < (*tuning).shrink_factor
        && (*tuning).shrink_factor <= 1 as libc::c_int as libc::c_float
        && (*tuning).shrink_threshold + epsilon < (*tuning).growth_threshold
    {
        return 1 as libc::c_int != 0;
    }
    (*table).tuning = &default_tuning;
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn compute_bucket_size(
    mut candidate: size_t,
    mut tuning: *const Hash_tuning,
) -> size_t {
    let mut current_block: u64;
    if !(*tuning).is_n_buckets {
        let mut new_candidate: libc::c_float = candidate as libc::c_float
            / (*tuning).growth_threshold;
        if 18446744073709551615 as libc::c_ulong as libc::c_float <= new_candidate {
            current_block = 8933918830699217881;
        } else {
            candidate = new_candidate as size_t;
            current_block = 12675440807659640239;
        }
    } else {
        current_block = 12675440807659640239;
    }
    match current_block {
        12675440807659640239 => {
            candidate = next_prime(candidate.try_into().unwrap());
            if !(::core::mem::size_of::<*mut hash_entry>() as libc::c_ulong
                != 0 as libc::c_int as libc::c_ulong
                && (if (9223372036854775807 as libc::c_long as libc::c_ulong)
                    < 18446744073709551615 as libc::c_ulong
                {
                    9223372036854775807 as libc::c_long as libc::c_ulong
                } else {
                    (18446744073709551615 as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                })
                    .wrapping_div(
                        ::core::mem::size_of::<*mut hash_entry>() as libc::c_ulong,
                    ) < candidate)
            {
                return candidate;
            }
        }
        _ => {}
    }
    *__errno_location() = 12 as libc::c_int;
    return 0 as libc::c_int as size_t;
}
#[no_mangle]
pub fn hash_initialize(
    candidate: usize,
    tuning: Option<&Hash_tuning>,
    hasher: Option<Hash_hasher>,
    comparator: Option<Hash_comparator>,
    data_freer: Hash_data_freer,
) -> Option<Box<Hash_table>> {
    let mut table = Box::new(Hash_table {
        tuning: tuning.unwrap_or_else(|| unsafe { &default_tuning }),
        n_buckets: 0,
        bucket: std::ptr::null_mut(),
        bucket_limit: std::ptr::null_mut(),
        n_buckets_used: 0,
        n_entries: 0,
        hasher: hasher.unwrap_or(Some(raw_hasher)),
        comparator: comparator.unwrap_or(Some(raw_comparator)),
        data_freer,
        free_entry_list: std::ptr::null_mut(),
    });

    if !unsafe { check_tuning(&mut *table) } {
        std::process::exit(22);
    }

    table.n_buckets = unsafe { compute_bucket_size(candidate.try_into().unwrap(), table.tuning) };
    if table.n_buckets == 0 {
        return None;
    }

    let bucket_size = table.n_buckets.try_into().unwrap();
    table.bucket = unsafe {
        libc::calloc(bucket_size, std::mem::size_of::<hash_entry>()) as *mut hash_entry
    };

    if table.bucket.is_null() {
        return None;
    }

    table.bucket_limit = unsafe { table.bucket.add(bucket_size) };

    Some(table)
}

#[no_mangle]
pub fn hash_clear(table: &mut Hash_table) {
    let bucket_limit = table.bucket_limit as usize;
    for i in 0..bucket_limit {
        let bucket = unsafe { &mut *table.bucket.add(i) };
        if !bucket.data.is_null() {
            if let Some(data_freer) = table.data_freer {
                unsafe {
                    data_freer(bucket.data);
                }
            }
            let mut cursor = unsafe { bucket.next };
            while !cursor.is_null() {
                let next_cursor = unsafe { &mut *cursor };
                if let Some(data_freer) = table.data_freer {
                    unsafe {
                        data_freer(next_cursor.data);
                    }
                }
                cursor = next_cursor.next;
                next_cursor.next = table.free_entry_list;
                table.free_entry_list = next_cursor as *mut _;
            }
            if let Some(data_freer) = table.data_freer {
                unsafe {
                    data_freer(bucket.data);
                }
            }
            bucket.data = std::ptr::null_mut();
            bucket.next = std::ptr::null_mut();
        }
    }
    table.n_buckets_used = 0;
    table.n_entries = 0;
}

#[no_mangle]
pub unsafe extern "C" fn hash_free(mut table: *mut Hash_table) {
    let mut bucket: *mut hash_entry = 0 as *mut hash_entry;
    let mut cursor: *mut hash_entry = 0 as *mut hash_entry;
    let mut next: *mut hash_entry = 0 as *mut hash_entry;
    let mut err: libc::c_int = *__errno_location();
    if ((*table).data_freer).is_some() && (*table).n_entries != 0 {
        bucket = (*table).bucket;
        while bucket < (*table).bucket_limit as *mut hash_entry {
            if !((*bucket).data).is_null() {
                cursor = bucket;
                while !cursor.is_null() {
                    ((*table).data_freer)
                        .expect("non-null function pointer")((*cursor).data);
                    cursor = (*cursor).next;
                }
            }
            bucket = bucket.offset(1);
            bucket;
        }
    }
    bucket = (*table).bucket;
    while bucket < (*table).bucket_limit as *mut hash_entry {
        cursor = (*bucket).next;
        while !cursor.is_null() {
            next = (*cursor).next;
            free(cursor as *mut libc::c_void);
            cursor = next;
        }
        bucket = bucket.offset(1);
        bucket;
    }
    cursor = (*table).free_entry_list;
    while !cursor.is_null() {
        next = (*cursor).next;
        free(cursor as *mut libc::c_void);
        cursor = next;
    }
    free((*table).bucket as *mut libc::c_void);
    free(table as *mut libc::c_void);
    *__errno_location() = err;
}
unsafe extern "C" fn allocate_entry(mut table: *mut Hash_table) -> *mut hash_entry {
    let mut new: *mut hash_entry = 0 as *mut hash_entry;
    if !((*table).free_entry_list).is_null() {
        new = (*table).free_entry_list;
        (*table).free_entry_list = (*new).next;
    } else {
        new = malloc(::core::mem::size_of::<hash_entry>() as libc::c_ulong)
            as *mut hash_entry;
    }
    return new;
}
unsafe extern "C" fn free_entry(mut table: *mut Hash_table, mut entry: *mut hash_entry) {
    (*entry).data = 0 as *mut libc::c_void;
    (*entry).next = (*table).free_entry_list;
    (*table).free_entry_list = entry;
}
unsafe extern "C" fn hash_find_entry(
    mut table: *mut Hash_table,
    mut entry: *const libc::c_void,
    mut bucket_head: *mut *mut hash_entry,
    mut delete: bool,
) -> *mut libc::c_void {
    let mut bucket: *mut hash_entry = safe_hasher(table, entry);
    let mut cursor: *mut hash_entry = 0 as *mut hash_entry;
    *bucket_head = bucket;
    if ((*bucket).data).is_null() {
        return 0 as *mut libc::c_void;
    }
    if entry == (*bucket).data as *const libc::c_void
        || ((*table).comparator)
            .expect("non-null function pointer")(entry, (*bucket).data) as libc::c_int
            != 0
    {
        let mut data: *mut libc::c_void = (*bucket).data;
        if delete {
            if !((*bucket).next).is_null() {
                let mut next: *mut hash_entry = (*bucket).next;
                *bucket = *next;
                free_entry(table, next);
            } else {
                (*bucket).data = 0 as *mut libc::c_void;
            }
        }
        return data;
    }
    cursor = bucket;
    while !((*cursor).next).is_null() {
        if entry == (*(*cursor).next).data as *const libc::c_void
            || ((*table).comparator)
                .expect("non-null function pointer")(entry, (*(*cursor).next).data)
                as libc::c_int != 0
        {
            let mut data_0: *mut libc::c_void = (*(*cursor).next).data;
            if delete {
                let mut next_0: *mut hash_entry = (*cursor).next;
                (*cursor).next = (*next_0).next;
                free_entry(table, next_0);
            }
            return data_0;
        }
        cursor = (*cursor).next;
    }
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn transfer_entries(
    mut dst: *mut Hash_table,
    mut src: *mut Hash_table,
    mut safe: bool,
) -> bool {
    let mut bucket: *mut hash_entry = 0 as *mut hash_entry;
    let mut cursor: *mut hash_entry = 0 as *mut hash_entry;
    let mut next: *mut hash_entry = 0 as *mut hash_entry;
    bucket = (*src).bucket;
    while bucket < (*src).bucket_limit as *mut hash_entry {
        if !((*bucket).data).is_null() {
            let mut data: *mut libc::c_void = 0 as *mut libc::c_void;
            let mut new_bucket: *mut hash_entry = 0 as *mut hash_entry;
            cursor = (*bucket).next;
            while !cursor.is_null() {
                data = (*cursor).data;
                new_bucket = safe_hasher(dst, data);
                next = (*cursor).next;
                if !((*new_bucket).data).is_null() {
                    (*cursor).next = (*new_bucket).next;
                    (*new_bucket).next = cursor;
                } else {
                    (*new_bucket).data = data;
                    (*dst).n_buckets_used = ((*dst).n_buckets_used).wrapping_add(1);
                    (*dst).n_buckets_used;
                    free_entry(dst, cursor);
                }
                cursor = next;
            }
            data = (*bucket).data;
            (*bucket).next = 0 as *mut hash_entry;
            if !safe {
                new_bucket = safe_hasher(dst, data);
                if !((*new_bucket).data).is_null() {
                    let mut new_entry: *mut hash_entry = allocate_entry(dst);
                    if new_entry.is_null() {
                        return 0 as libc::c_int != 0;
                    }
                    (*new_entry).data = data;
                    (*new_entry).next = (*new_bucket).next;
                    (*new_bucket).next = new_entry;
                } else {
                    (*new_bucket).data = data;
                    (*dst).n_buckets_used = ((*dst).n_buckets_used).wrapping_add(1);
                    (*dst).n_buckets_used;
                }
                (*bucket).data = 0 as *mut libc::c_void;
                (*src).n_buckets_used = ((*src).n_buckets_used).wrapping_sub(1);
                (*src).n_buckets_used;
            }
        }
        bucket = bucket.offset(1);
        bucket;
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn hash_rehash(
    mut table: *mut Hash_table,
    mut candidate: size_t,
) -> bool {
    let mut storage: Hash_table = Hash_table {
        bucket: 0 as *mut hash_entry,
        bucket_limit: 0 as *const hash_entry,
        n_buckets: 0,
        n_buckets_used: 0,
        n_entries: 0,
        tuning: 0 as *const Hash_tuning,
        hasher: None,
        comparator: None,
        data_freer: None,
        free_entry_list: 0 as *mut hash_entry,
    };
    let mut new_table: *mut Hash_table = 0 as *mut Hash_table;
    let mut new_size: size_t = compute_bucket_size(candidate, (*table).tuning);
    if new_size == 0 {
        return 0 as libc::c_int != 0;
    }
    if new_size == (*table).n_buckets {
        return 1 as libc::c_int != 0;
    }
    new_table = &mut storage;
    (*new_table)
        .bucket = calloc(new_size, ::core::mem::size_of::<hash_entry>() as libc::c_ulong)
        as *mut hash_entry;
    if ((*new_table).bucket).is_null() {
        return 0 as libc::c_int != 0;
    }
    (*new_table).n_buckets = new_size;
    (*new_table).bucket_limit = ((*new_table).bucket).offset(new_size as isize);
    (*new_table).n_buckets_used = 0 as libc::c_int as size_t;
    (*new_table).n_entries = 0 as libc::c_int as size_t;
    (*new_table).tuning = (*table).tuning;
    (*new_table).hasher = (*table).hasher;
    (*new_table).comparator = (*table).comparator;
    (*new_table).data_freer = (*table).data_freer;
    (*new_table).free_entry_list = (*table).free_entry_list;
    if transfer_entries(new_table, table, 0 as libc::c_int != 0) {
        free((*table).bucket as *mut libc::c_void);
        (*table).bucket = (*new_table).bucket;
        (*table).bucket_limit = (*new_table).bucket_limit;
        (*table).n_buckets = (*new_table).n_buckets;
        (*table).n_buckets_used = (*new_table).n_buckets_used;
        (*table).free_entry_list = (*new_table).free_entry_list;
        return 1 as libc::c_int != 0;
    }
    let mut err: libc::c_int = *__errno_location();
    (*table).free_entry_list = (*new_table).free_entry_list;
    if !(transfer_entries(table, new_table, 1 as libc::c_int != 0) as libc::c_int != 0
        && transfer_entries(table, new_table, 0 as libc::c_int != 0) as libc::c_int != 0)
    {
        abort();
    }
    free((*new_table).bucket as *mut libc::c_void);
    *__errno_location() = err;
    return 0 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn hash_insert_if_absent(
    mut table: *mut Hash_table,
    mut entry: *const libc::c_void,
    mut matched_ent: *mut *const libc::c_void,
) -> libc::c_int {
    let mut data: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut bucket: *mut hash_entry = 0 as *mut hash_entry;
    if entry.is_null() {
        abort();
    }
    data = hash_find_entry(table, entry, &mut bucket, 0 as libc::c_int != 0);
    if !data.is_null() {
        if !matched_ent.is_null() {
            *matched_ent = data;
        }
        return 0 as libc::c_int;
    }
    if (*table).n_buckets_used as libc::c_float
        > (*(*table).tuning).growth_threshold * (*table).n_buckets as libc::c_float
    {
        check_tuning(table);
        if (*table).n_buckets_used as libc::c_float
            > (*(*table).tuning).growth_threshold * (*table).n_buckets as libc::c_float
        {
            let mut tuning: *const Hash_tuning = (*table).tuning;
            let mut candidate: libc::c_float = if (*tuning).is_n_buckets as libc::c_int
                != 0
            {
                (*table).n_buckets as libc::c_float * (*tuning).growth_factor
            } else {
                (*table).n_buckets as libc::c_float * (*tuning).growth_factor
                    * (*tuning).growth_threshold
            };
            if 18446744073709551615 as libc::c_ulong as libc::c_float <= candidate {
                *__errno_location() = 12 as libc::c_int;
                return -(1 as libc::c_int);
            }
            if !hash_rehash(table, candidate as size_t) {
                return -(1 as libc::c_int);
            }
            if !(hash_find_entry(table, entry, &mut bucket, 0 as libc::c_int != 0))
                .is_null()
            {
                abort();
            }
        }
    }
    if !((*bucket).data).is_null() {
        let mut new_entry: *mut hash_entry = allocate_entry(table);
        if new_entry.is_null() {
            return -(1 as libc::c_int);
        }
        (*new_entry).data = entry as *mut libc::c_void;
        (*new_entry).next = (*bucket).next;
        (*bucket).next = new_entry;
        (*table).n_entries = ((*table).n_entries).wrapping_add(1);
        (*table).n_entries;
        return 1 as libc::c_int;
    }
    (*bucket).data = entry as *mut libc::c_void;
    (*table).n_entries = ((*table).n_entries).wrapping_add(1);
    (*table).n_entries;
    (*table).n_buckets_used = ((*table).n_buckets_used).wrapping_add(1);
    (*table).n_buckets_used;
    return 1 as libc::c_int;
}
#[no_mangle]
pub fn hash_insert(
    table: &mut Hash_table,
    entry: &libc::c_void,
) -> Option<*const libc::c_void> {
    let mut matched_ent: *const libc::c_void = std::ptr::null();
    
    unsafe {
        let err = hash_insert_if_absent(table, entry, &mut matched_ent);
        
        if err == -1 {
            None
        } else {
            Some(if err == 0 { matched_ent } else { entry })
        }
    }
}

#[no_mangle]
pub fn hash_remove(
    table: &mut Hash_table,
    entry: *const libc::c_void,
) -> *mut libc::c_void {
    let mut data: *mut libc::c_void;
    let mut bucket: *mut hash_entry = std::ptr::null_mut();

    unsafe {
        data = hash_find_entry(table, entry, &mut bucket, true);
        if data.is_null() {
            return std::ptr::null_mut();
        }

        table.n_entries = table.n_entries.wrapping_sub(1);
        
        if (*bucket).data.is_null() {
            table.n_buckets_used = table.n_buckets_used.wrapping_sub(1);
            
            if (table.n_buckets_used as f32) < (unsafe { (*table.tuning).shrink_threshold } * table.n_buckets as f32) {
                check_tuning(table);
                if (table.n_buckets_used as f32) < (unsafe { (*table.tuning).shrink_threshold } * table.n_buckets as f32) {
                    let tuning = unsafe { &*table.tuning };
                    let candidate: usize = if tuning.is_n_buckets {
                        (table.n_buckets as f32 * tuning.shrink_factor) as usize
                    } else {
                        (table.n_buckets as f32 * tuning.shrink_factor * tuning.growth_threshold) as usize
                    };
                    
                    if !hash_rehash(table, candidate.try_into().unwrap()) {
                        let mut cursor = table.free_entry_list;
                        while !cursor.is_null() {
                            let next = unsafe { (*cursor).next };
                            free(cursor as *mut libc::c_void);
                            cursor = next;
                        }
                        table.free_entry_list = std::ptr::null_mut();
                    }
                }
            }
        }
    }
    data
}

#[no_mangle]
pub fn hash_delete(
    table: &mut Hash_table,
    entry: &libc::c_void,
) -> *mut libc::c_void {
    hash_remove(table, entry)
}

