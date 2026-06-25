use chess::board::Board;
use vizia::prelude::*;

fn main() {
    Application::new(|cx| {
        let board = Board::new();

        HStack::new(cx, |cx| {
            for square in board.squares {
                Label::new(cx, format!("Square: {0}:{1}", square.pos_x, square.pos_y))
                    .width(Pixels(50.0));
            }
        })
        .alignment(Alignment::Center)
        .horizontal_gap(Pixels(50.0));
    })
    .title("Peter Chess")
    .inner_size((400, 100))
    .run();
}
