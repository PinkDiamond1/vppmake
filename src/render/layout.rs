use super::{Body, Buf};
use crate::{bbcode, css};
use std::fmt::Display;

pub fn render_shell<B: Body>(buf: Buf, body: B) {
    css! {
        shell {
            {{ "--w: 608px;" }};
            {{ "--h: 404.44px;" }};

            font_family: "'press start 2p'";
        }
    }

    bbcode! {
        in {{ buf }};

        center {
            div {{ shell }} {
                yield {{ body }};
            }
        }
    }
}

pub fn render_block<B: Body>(buf: Buf, body: B) {
    css! {
        outer {
            display: "inline-block";

            padding: "2px";
            max_width: "100%";

            border: "2px solid black";
            border_radius: "4px";

            background_color: "white";
            color: "black";
        }

        inner {
            display: "flex";
            align_items: "flex-end";

            position: "relative";
            overflow_x: "scroll";
            max_width: "100%";

            border: "2px solid black";
            border_radius: "4px";
        }
    }

    bbcode! {
        in {{ buf }};

        div {{ outer }} {
            div {{ inner }} {
                yield {{ body }};
            }
        }
    }
}

pub fn render_sub_block<B: Body>(buf: Buf, body: B) {
    css! {
        outer {
            margin_top: "0.25rem";
            padding: "2px";
            width: "608px";
            max_width: "100%";

            border: "2px solid black";
            border_radius: "4px";

            background_color: "white";
            color: "black";
            font_family: "'press start 2p'";
            text_align: "left";
        }

        inner {
            padding: "1rem";

            border: "2px solid black";
            border_radius: "4px";

            background_color: "white";
        }
    }

    bbcode! {
        in {{ buf }};

        div {{ outer }} {
            div {{ inner }} {
                yield {{ body }};
            }
        }
    }
}

pub fn render_bg_img<T: Display>(buf: Buf, img: T) {
    css! {
        cimg_css {
            width: "var(--w)";
            height: "var(--h)";

            max_width: "unset !important";
        }
    }

    bbcode! {
        in {{ buf }};

        cimg {{ cimg_css }} {
            {{ img }};
        }
    }
}

pub fn render_tab(buf: Buf, slug: &str, name: &str) {
    css! {
        outer {
            display: "inline-block";

            padding: "2px";
            padding_bottom: "0";

            border: "2px solid black";
            border_bottom: "0";
            border_top_left_radius: "4px";
            border_top_right_radius: "4px";

            background_color: "white";
            color: "black !important";
            font_size: "80%";
        }

        inner {
            padding: "6px 9px";
            padding_bottom: "0";

            border: "2px solid black";
            border_bottom: 0;
            border_top_left_radius: "4px";
            border_top_right_radius: "4px";
        }
    }

    bbcode! {
        in {{ buf }};

        tab {{ slug }} {
            div {{ outer }} {
                div {{ inner }} {
                    {{ name }};
                }
            }
        }
    }
}

pub fn render_tab_row<B: Body>(buf: Buf, body: B) {
    css! {
        row {
            width: "var(--w)";
            max_width: "100%";

            text_align: "left";
        }
    }

    bbcode! {
        in {{ buf }};

        div {{ row }} {
            tabgroup {
                yield {{ body }};
            }
        }
    }
}
