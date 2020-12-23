use std::{
    collections::BTreeMap,
    io::{self, Read},
    marker::PhantomPinned,
    pin::Pin,
    ptr,
};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    unsafe {
        let mut cups: Vec<Pin<Box<Cup>>> = Vec::new();

        let mut max_val = usize::MIN;

        for c in input.chars() {
            let val = c.to_digit(10).unwrap() as usize;

            max_val = max_val.max(val);

            let mut cup = Box::pin(Cup {
                val,
                next: ptr::null_mut(),
                _pin: PhantomPinned,
            });

            if let Some(prev) = cups
                .last_mut()
                .map(|prev| prev.as_mut().get_unchecked_mut())
            {
                (*prev).next = cup.as_mut().get_unchecked_mut();
            }

            cups.push(cup);
        }

        let current_val = (*cups[0].as_ref().get_ref()).val;

        (cups.last_mut().unwrap().as_mut().get_unchecked_mut()).next =
            cups.first_mut().unwrap().as_mut().get_unchecked_mut();

        let mut cups: BTreeMap<usize, Pin<Box<Cup>>> =
            cups.into_iter().map(|cup| ((*cup).val, cup)).collect();

        let mut current = cups
            .get_mut(&current_val)
            .unwrap()
            .as_mut()
            .get_unchecked_mut();

        for _ in 0..100 {
            let picked_up_first = (*current).next;
            let picked_up_last = (*(*picked_up_first).next).next;
            (*current).next = (*picked_up_last).next;

            let picked_up_vals = [
                (*picked_up_first).val,
                (*(*picked_up_first).next).val,
                (*picked_up_last).val,
            ];

            let mut dest_val = (*current).val - 1;
            if dest_val == 0 {
                dest_val = max_val;
            }

            while picked_up_vals.contains(&dest_val) {
                dest_val -= 1;
                if dest_val == 0 {
                    dest_val = max_val;
                }
            }

            current = &mut *(*current).next;

            let dest = cups
                .get_mut(&dest_val)
                .unwrap()
                .as_mut()
                .get_unchecked_mut();

            (*picked_up_last).next = (*dest).next;
            (*dest).next = picked_up_first;
        }

        let mut part_1 = String::new();
        let mut cup = (*cups.get_mut(&1).unwrap().as_ref().get_ref()).next;
        while (*cup).val != 1 {
            part_1.push_str(&(*cup).val.to_string());
            cup = (*cup).next;
        }

        println!("Part 1: {}", part_1);
    }

    unsafe {
        const NUM_CUPS: usize = 1_000_000;

        let mut cups: Vec<Pin<Box<Cup>>> = Vec::with_capacity(NUM_CUPS);

        let mut max_val = usize::MIN;

        for c in input.chars() {
            let val = c.to_digit(10).unwrap() as usize;

            max_val = max_val.max(val);

            let mut cup = Box::pin(Cup {
                val,
                next: ptr::null_mut(),
                _pin: PhantomPinned,
            });

            if let Some(prev) = cups
                .last_mut()
                .map(|prev| prev.as_mut().get_unchecked_mut())
            {
                (*prev).next = cup.as_mut().get_unchecked_mut();
            }

            cups.push(cup);
        }

        for val in (max_val + 1)..=NUM_CUPS {
            let mut cup = Box::pin(Cup {
                val,
                next: ptr::null_mut(),
                _pin: PhantomPinned,
            });

            if let Some(prev) = cups
                .last_mut()
                .map(|prev| prev.as_mut().get_unchecked_mut())
            {
                (*prev).next = cup.as_mut().get_unchecked_mut();
            }

            cups.push(cup);
        }

        let current_val = (*cups[0].as_ref().get_ref()).val;

        (cups.last_mut().unwrap().as_mut().get_unchecked_mut()).next =
            cups.first_mut().unwrap().as_mut().get_unchecked_mut();

        let mut cups: BTreeMap<usize, Pin<Box<Cup>>> =
            cups.into_iter().map(|cup| ((*cup).val, cup)).collect();

        let mut current = cups
            .get_mut(&current_val)
            .unwrap()
            .as_mut()
            .get_unchecked_mut();

        for _ in 0..10_000_000 {
            let picked_up_first = (*current).next;
            let picked_up_last = (*(*picked_up_first).next).next;
            (*current).next = (*picked_up_last).next;

            let picked_up_vals = [
                (*picked_up_first).val,
                (*(*picked_up_first).next).val,
                (*picked_up_last).val,
            ];

            let mut dest_val = (*current).val - 1;
            if dest_val == 0 {
                dest_val = NUM_CUPS;
            }

            while picked_up_vals.contains(&dest_val) {
                dest_val -= 1;
                if dest_val == 0 {
                    dest_val = NUM_CUPS;
                }
            }

            current = &mut *(*current).next;

            let dest = cups
                .get_mut(&dest_val)
                .unwrap()
                .as_mut()
                .get_unchecked_mut();

            (*picked_up_last).next = (*dest).next;
            (*dest).next = picked_up_first;
        }

        let next = (*cups.get_mut(&1).unwrap().as_ref().get_ref()).next;
        let part_2: u128 = (*next).val as u128 * (*(*next).next).val as u128;
        println!("Part 2: {}", part_2);
    }
}

#[derive(Debug, Clone)]
struct Cup {
    val: usize,
    next: *mut Cup,
    _pin: PhantomPinned,
}
