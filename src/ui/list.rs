use embedded_graphics::mono_font::{ascii::FONT_6X10, MonoTextStyle};
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{PrimitiveStyle, Rectangle};
use embedded_graphics::text::Text;

use super::{clear_strip, TextBuf, SCREEN_H, SCREEN_W};

const BG: Rgb565 = Rgb565::BLACK;
const FG: Rgb565 = Rgb565::WHITE;
const TITLE_BG: Rgb565 = Rgb565::new(9, 19, 9);

const ROW_HEIGHT: i32 = FONT_6X10.character_size.height as i32 + 4;
const TITLE_HEIGHT: i32 = ROW_HEIGHT;
const LIST_TOP: i32 = TITLE_HEIGHT;
const VISIBLE_ROWS: usize = ((SCREEN_H - LIST_TOP) / ROW_HEIGHT) as usize;
const RIGHT_MARGIN: i32 = 4;

const ARROW_GAP: i32 = 6;

const TITLE_MAX: usize = 16;
const VALUE_MAX: usize = 18;

pub(crate) trait ListSource {
    fn row_count(&mut self) -> usize;
    fn label(&mut self, index: usize, w: &mut dyn core::fmt::Write);
    fn value(&mut self, _index: usize, _w: &mut dyn core::fmt::Write) -> bool {
        false
    }
    fn cursor(&mut self, _index: usize) -> Option<usize> {
        None
    }
}

pub(crate) struct Cache {
    title: Option<TitleKey>,
    selected: Option<usize>,
    total: Option<usize>,
    scroll_top: Option<usize>,
    sel: Option<SelRender>,
}

#[derive(Clone, Copy, PartialEq)]
struct TitleKey {
    len: u8,
    buf: [u8; TITLE_MAX],
}

impl TitleKey {
    fn of(title: &str) -> Self {
        let b = title.as_bytes();
        let n = b.len().min(TITLE_MAX);
        let mut buf = [0u8; TITLE_MAX];
        buf[..n].copy_from_slice(&b[..n]);
        TitleKey { len: n as u8, buf }
    }
}

#[derive(Clone, Copy, PartialEq)]
struct SelRender {
    show_arrows: bool,
    value: TextBuf<VALUE_MAX>,
    cursor: Option<usize>,
}

fn render_sel(
    source: &mut dyn ListSource,
    selected: usize,
    show_arrows: bool,
) -> SelRender {
    let mut value: TextBuf<VALUE_MAX> = TextBuf::new();
    let has_value = source.value(selected, &mut value);
    let cursor = if has_value {
        source.cursor(selected)
    } else {
        None
    };
    SelRender {
        show_arrows,
        value,
        cursor,
    }
}

impl Cache {
    pub fn new() -> Self {
        Cache {
            title: None,
            selected: None,
            total: None,
            scroll_top: None,
            sel: None,
        }
    }

    fn matches_title(&self, title: &str) -> bool {
        match self.title {
            Some(k) => {
                let n = k.len as usize;
                title.len() == n && k.buf[..n] == title.as_bytes()[..n]
            }
            None => false,
        }
    }
}

pub(crate) fn draw_list<D>(
    lcd: &mut D,
    title: &str,
    source: &mut dyn ListSource,
    selected: usize,
    show_arrows: bool,
    cache: &mut Cache,
) where
    D: DrawTarget<Color = Rgb565>,
{
    let total = source.row_count();
    let top = scroll_top(total, selected);

    let title_changed = !cache.matches_title(title);
    let first = cache.selected.is_none() || title_changed;

    if first {
        draw_full(lcd, title, source, selected, show_arrows, total, top);
    } else {
        let prev_selected = cache.selected.unwrap();
        let prev_top = cache.scroll_top.unwrap();
        let prev_total = cache.total.unwrap();

        if top != prev_top || total != prev_total {
            clear_body(lcd);
            for slot in 0..VISIBLE_ROWS {
                draw_row(
                    lcd,
                    source,
                    top + slot,
                    selected,
                    show_arrows,
                    total,
                    slot,
                );
            }
        } else if selected != prev_selected {
            draw_row(
                lcd,
                source,
                prev_selected,
                selected,
                show_arrows,
                total,
                slot_of(prev_selected, top),
            );
            draw_row(
                lcd,
                source,
                selected,
                selected,
                show_arrows,
                total,
                slot_of(selected, top),
            );
        } else {
            if cache.sel != Some(render_sel(source, selected, show_arrows)) {
                draw_row(
                    lcd,
                    source,
                    selected,
                    selected,
                    show_arrows,
                    total,
                    slot_of(selected, top),
                );
            }
        }
    }

    cache.title = Some(TitleKey::of(title));
    cache.selected = Some(selected);
    cache.total = Some(total);
    cache.scroll_top = Some(top);
    cache.sel = Some(render_sel(source, selected, show_arrows));
}

fn slot_of(index: usize, top: usize) -> usize {
    index - top
}

fn draw_full<D>(
    lcd: &mut D,
    title: &str,
    source: &mut dyn ListSource,
    selected: usize,
    show_arrows: bool,
    total: usize,
    top: usize,
) where
    D: DrawTarget<Color = Rgb565>,
{
    clear_strip(lcd, 0, SCREEN_H);

    draw_title(lcd, title);

    for slot in 0..VISIBLE_ROWS {
        draw_row(lcd, source, top + slot, selected, show_arrows, total, slot);
    }
}

fn clear_body<D>(lcd: &mut D)
where
    D: DrawTarget<Color = Rgb565>,
{
    Rectangle::new(
        Point::new(0, LIST_TOP),
        Size::new(SCREEN_W as u32, (SCREEN_H - LIST_TOP) as u32),
    )
    .into_styled(PrimitiveStyle::with_fill(BG))
    .draw(lcd)
    .ok();
}

fn draw_title<D>(lcd: &mut D, title: &str)
where
    D: DrawTarget<Color = Rgb565>,
{
    Rectangle::new(
        Point::new(0, 0),
        Size::new(SCREEN_W as u32, TITLE_HEIGHT as u32),
    )
    .into_styled(PrimitiveStyle::with_fill(TITLE_BG))
    .draw(lcd)
    .ok();
    draw_centered(lcd, title, FONT_6X10.baseline as i32 + 2);
}

fn draw_row<D>(
    lcd: &mut D,
    source: &mut dyn ListSource,
    index: usize,
    selected: usize,
    show_arrows: bool,
    total: usize,
    slot: usize,
) where
    D: DrawTarget<Color = Rgb565>,
{
    let row_top = LIST_TOP + slot as i32 * ROW_HEIGHT;
    Rectangle::new(
        Point::new(0, row_top),
        Size::new(SCREEN_W as u32, ROW_HEIGHT as u32),
    )
    .into_styled(PrimitiveStyle::with_fill(BG))
    .draw(lcd)
    .ok();

    if index >= total {
        return; // blank row below the end of a shorter list
    }

    let is_selected = index == selected;
    if is_selected {
        Rectangle::new(
            Point::new(0, row_top),
            Size::new(SCREEN_W as u32, ROW_HEIGHT as u32),
        )
        .into_styled(PrimitiveStyle::with_fill(FG))
        .draw(lcd)
        .ok();
    }
    let fg = if is_selected { BG } else { FG };
    let baseline = row_top + FONT_6X10.baseline as i32 + 2;

    let mut label: TextBuf<20> = TextBuf::new();
    source.label(index, &mut label);
    let label_is_empty = label.as_str().is_empty();

    if !label_is_empty {
        Text::new(
            label.as_str(),
            Point::new(4, baseline),
            MonoTextStyle::new(&FONT_6X10, fg),
        )
        .draw(lcd)
        .ok();
    }

    let mut value: TextBuf<18> = TextBuf::new();
    if source.value(index, &mut value) {
        if let Some(cursor) = source.cursor(index) {
            if label_is_empty {
                draw_value_with_cursor_left(
                    lcd,
                    value.as_str(),
                    cursor,
                    row_top,
                    baseline,
                    fg,
                );
            } else {
                draw_value_with_cursor(
                    lcd,
                    value.as_str(),
                    cursor,
                    row_top,
                    baseline,
                    fg,
                );
            }
        } else if is_selected && show_arrows {
            draw_editing_value(lcd, value.as_str(), row_top, fg);
        } else if label_is_empty {
            draw_value_left(lcd, value.as_str(), baseline, fg);
        } else {
            draw_value(lcd, value.as_str(), baseline, fg);
        }
    }
}

fn scroll_top(total: usize, selected: usize) -> usize {
    if total <= VISIBLE_ROWS {
        return 0;
    }
    let half = VISIBLE_ROWS / 2;
    selected.saturating_sub(half).min(total - VISIBLE_ROWS)
}

fn draw_centered<D>(lcd: &mut D, text: &str, baseline_y: i32)
where
    D: DrawTarget<Color = Rgb565>,
{
    let width =
        text.chars().count() as i32 * FONT_6X10.character_size.width as i32;
    let x = (SCREEN_W - width) / 2;
    Text::new(
        text,
        Point::new(x, baseline_y),
        MonoTextStyle::new(&FONT_6X10, FG),
    )
    .draw(lcd)
    .ok();
}

fn draw_value<D>(lcd: &mut D, text: &str, baseline_y: i32, fg: Rgb565)
where
    D: DrawTarget<Color = Rgb565>,
{
    let width =
        text.chars().count() as i32 * FONT_6X10.character_size.width as i32;
    let x = SCREEN_W - RIGHT_MARGIN - width;
    Text::new(
        text,
        Point::new(x, baseline_y),
        MonoTextStyle::new(&FONT_6X10, fg),
    )
    .draw(lcd)
    .ok();
}

fn draw_value_left<D>(lcd: &mut D, text: &str, baseline_y: i32, fg: Rgb565)
where
    D: DrawTarget<Color = Rgb565>,
{
    Text::new(
        text,
        Point::new(4, baseline_y),
        MonoTextStyle::new(&FONT_6X10, fg),
    )
    .draw(lcd)
    .ok();
}

fn draw_value_with_cursor<D>(
    lcd: &mut D,
    text: &str,
    cursor_index: usize,
    row_top: i32,
    baseline_y: i32,
    fg: Rgb565,
) where
    D: DrawTarget<Color = Rgb565>,
{
    draw_value(lcd, text, baseline_y, fg);

    let char_w = FONT_6X10.character_size.width as i32;
    let width = text.chars().count() as i32 * char_w;
    let x0 = SCREEN_W - RIGHT_MARGIN - width;

    draw_cursor_box(lcd, text, cursor_index, x0, row_top, baseline_y, fg);
}

fn draw_value_with_cursor_left<D>(
    lcd: &mut D,
    text: &str,
    cursor_index: usize,
    row_top: i32,
    baseline_y: i32,
    fg: Rgb565,
) where
    D: DrawTarget<Color = Rgb565>,
{
    draw_value_left(lcd, text, baseline_y, fg);
    draw_cursor_box(lcd, text, cursor_index, 4, row_top, baseline_y, fg);
}

fn draw_cursor_box<D>(
    lcd: &mut D,
    text: &str,
    cursor_index: usize,
    x0: i32,
    row_top: i32,
    baseline_y: i32,
    fg: Rgb565,
) where
    D: DrawTarget<Color = Rgb565>,
{
    let char_w = FONT_6X10.character_size.width as i32;

    let ch = text.chars().nth(cursor_index).unwrap_or(' ');
    let cursor_x = x0 + cursor_index as i32 * char_w;
    let inverted = if fg == FG { BG } else { FG };
    Rectangle::new(
        Point::new(cursor_x, row_top),
        Size::new(char_w as u32, ROW_HEIGHT as u32),
    )
    .into_styled(PrimitiveStyle::with_fill(fg))
    .draw(lcd)
    .ok();

    let mut ch_buf = [0u8; 4];
    Text::new(
        ch.encode_utf8(&mut ch_buf),
        Point::new(cursor_x, baseline_y),
        MonoTextStyle::new(&FONT_6X10, inverted),
    )
    .draw(lcd)
    .ok();
}

/// `'^'`/`'v'` glyphs stand in for up/down arrow triangles
fn draw_editing_value<D>(lcd: &mut D, text: &str, row_top: i32, fg: Rgb565)
where
    D: DrawTarget<Color = Rgb565>,
{
    let char_w = FONT_6X10.character_size.width as i32;
    let value_width = text.chars().count() as i32 * char_w;
    let total_width = char_w + ARROW_GAP + value_width + ARROW_GAP + char_w;
    let start_x = SCREEN_W - RIGHT_MARGIN - total_width;
    let baseline = row_top + FONT_6X10.baseline as i32 + 2;
    let style = MonoTextStyle::new(&FONT_6X10, fg);

    Text::new("^", Point::new(start_x, baseline), style)
        .draw(lcd)
        .ok();

    let value_x = start_x + char_w + ARROW_GAP;
    Text::new(text, Point::new(value_x, baseline), style)
        .draw(lcd)
        .ok();

    let down_x = value_x + value_width + ARROW_GAP;
    Text::new("v", Point::new(down_x, baseline), style)
        .draw(lcd)
        .ok();
}
