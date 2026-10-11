//! Aggregate tray indicators never include account or contact identity.
use std::sync::atomic::{AtomicU32, Ordering};

static STATE: AtomicU32 = AtomicU32::new(0);

pub fn publish(count: u32, vip: bool, numbers: bool) -> bool {
    let next = count.min(100) | (u32::from(vip) << 8) | (u32::from(numbers) << 9);
    STATE.swap(next, Ordering::Relaxed) != next
}

pub fn draw(size: usize) -> Vec<u8> {
    let state = STATE.load(Ordering::Relaxed);
    pixels(size, state & 255, state & 256 != 0, state & 512 != 0)
}

fn pixels(size: usize, count: u32, vip: bool, numbers: bool) -> Vec<u8> {
    let mut rgba = crate::util::app_icon_rgba(size);
    if size < 16 || (!vip && (!numbers || count == 0)) {
        return rgba;
    }
    // A high-contrast corner marker leaves the recognizable phone icon intact.
    let label = if numbers && count > 0 {
        if count > 99 {
            "99+".to_owned()
        } else {
            count.to_string()
        }
    } else {
        "!".to_owned()
    };
    let scale = (size / 24).max(1);
    let width = label.len() * 4 * scale + 3 * scale;
    let height = 7 * scale;
    let left = size.saturating_sub(width);
    let top = size.saturating_sub(height);
    let background = if vip {
        [255, 196, 48, 255]
    } else {
        [158, 20, 38, 255]
    };
    let foreground = if vip {
        [24, 24, 24, 255]
    } else {
        [255, 255, 255, 255]
    };
    let mut put = |x: usize, y: usize, color: [u8; 4]| {
        if x < size && y < size {
            rgba[(y * size + x) * 4..(y * size + x) * 4 + 4].copy_from_slice(&color);
        }
    };
    for y in top..size {
        for x in left..size {
            put(x, y, background);
        }
    }
    for (index, ch) in label.bytes().enumerate() {
        let rows = match ch {
            b'0' => [7, 5, 5, 5, 7],
            b'1' => [2, 6, 2, 2, 7],
            b'2' => [7, 1, 7, 4, 7],
            b'3' => [7, 1, 7, 1, 7],
            b'4' => [5, 5, 7, 1, 1],
            b'5' => [7, 4, 7, 1, 7],
            b'6' => [7, 4, 7, 5, 7],
            b'7' => [7, 1, 2, 2, 2],
            b'8' => [7, 5, 7, 5, 7],
            b'9' => [7, 5, 7, 1, 7],
            b'+' => [0, 2, 7, 2, 0],
            _ => [2, 2, 2, 0, 2],
        };
        for (row, bits) in rows.into_iter().enumerate() {
            for col in 0..3 {
                if bits & (1 << (2 - col)) != 0 {
                    for dy in 0..scale {
                        for dx in 0..scale {
                            put(
                                left + (2 + index * 4 + col) * scale + dx,
                                top + (row + 1) * scale + dy,
                                foreground,
                            );
                        }
                    }
                }
            }
        }
    }
    rgba
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn indicators_fit_small_and_retina_icons_and_keep_the_default_when_quiet() {
        for size in [16, 20, 24, 32, 36, 48, 64] {
            let default = crate::util::app_icon_rgba(size);
            assert_eq!(pixels(size, 0, false, true), default);
            for count in [1, 9, 10, 99, 100, u32::MAX] {
                let icon = pixels(size, count, true, true);
                assert_eq!(icon.len(), size * size * 4);
                assert_ne!(icon, default);
            }
        }
    }
}
