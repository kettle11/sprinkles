fn main() {
    let mut x_position = 200.0;
    let mut y_position = 30.0;

    let mut x_direction = 1.0;
    let mut y_direction = 1.0;

    let speed = 5.0;

    sprinkles::set_event_handler(move |context, event| match event {
        sprinkles::Event::Draw => {
            x_position += x_direction * speed;
            y_position += y_direction * speed;
            if x_position <= 0.0 || x_position >= 500.0 {
                x_direction *= -1.0;
            }
            if y_position <= 0.0 || y_position >= 500.0 {
                y_direction *= -1.0;
            }
            context.clear_screen_to_color(0.0, 0.0, 0.3, 1.0);
            context.draw_rectangle(x_position, y_position, 100., 100., 1.0, 0.0, 0.0, 1.0);
        }
        _ => {}
    })
}
