use spliq_core::{
    BasicRenderer, Constraints, FontStyle, ImageData, LayoutResult, MessageQueue, Path, Point,
    Rect, Renderer, Size, Style, Widget,
};

#[test]
fn message_queue_is_fifo() {
    let mut queue = MessageQueue::new();
    queue.send(1);
    queue.send(2);
    queue.send(3);

    assert_eq!(queue.pop(), Some(1));
    assert_eq!(queue.pop(), Some(2));
    assert_eq!(queue.pop(), Some(3));
    assert_eq!(queue.pop(), None);
}

#[test]
fn constraints_clamp_size_within_bounds() {
    let constraints = Constraints::new(10.0, 100.0, 20.0, 200.0);
    let clamped = constraints.clamp_size(Size::new(150.0, 5.0));

    assert_eq!(clamped, Size::new(100.0, 20.0));
}

#[test]
fn blanket_renderer_impl_supports_dyn_usage() {
    #[derive(Default)]
    struct RecordingRenderer {
        rects: usize,
    }

    impl BasicRenderer for RecordingRenderer {
        fn draw_rect(&mut self, _rect: Rect, _style: &Style) {
            self.rects += 1;
        }

        fn draw_line(&mut self, _start: Point, _end: Point, _style: &Style) {}

        fn draw_ellipse(&mut self, _center: Point, _radii: Size, _style: &Style) {}

        fn draw_path(&mut self, _path: &Path, _style: &Style) {}

        fn draw_text(&mut self, _text: &str, _pos: Point, _style: &FontStyle) {}

        fn draw_image(&mut self, _image: &ImageData, _rect: Rect) {}
    }

    struct TestWidget;

    impl Widget for TestWidget {
        fn layout(&self, constraints: Constraints) -> LayoutResult {
            LayoutResult::new(constraints.clamp_size(Size::new(40.0, 20.0)))
        }

        fn render(&self, renderer: &mut dyn Renderer, rect: Rect) {
            renderer.draw_rect(rect, &Style::default());
        }
    }

    let widget = TestWidget;
    let mut renderer = RecordingRenderer::default();
    let layout = widget.layout(Constraints::tight(Size::new(40.0, 20.0)));
    widget.render(&mut renderer, Rect::new(Point::new(0.0, 0.0), layout.size));

    assert_eq!(renderer.rects, 1);
}
