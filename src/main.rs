use plotters::prelude::*;
use std::f64::consts::{E, PI};
use std::io; //imported libraries//
fn main() -> Result<(), Box<dyn std::error::Error>> {
    loop{
        println!("What is your magic level? Ctrl + C to stop");
        let mut magic_level = String::new();

        io::stdin()
            .read_line(&mut magic_level)
            .expect("Failed to read or incorrect input"); //takes input and checks if it is expected input//
        println!("You input {}", magic_level);
        let magic_level: f64 = match magic_level.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, //trims away garbage sent in like spacebar etc//
        };
    let root = BitMapBackend::new("graph.png", (640, 480)).into_drawing_area();{ //creates the file to put the graph in//
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("Magic Missle", ("sans-serif", 50).into_font())
        .margin(5i32)
        .x_label_area_size(30i32)
        .y_label_area_size(30i32)
        .build_cartesian_2d(-10.0f64..10.0f64, 0f64..5f64)?; //settings for graph, first two values are start of x and end of x//
    chart.configure_mesh().draw()?;                          //second two are start of y and end of y// 
    chart 
        .draw_series(LineSeries::new(
            (-2500000..=2500000).map(|x| x as f64 / 100000.0).map(|x| { //change 100000.0 for different amounts of precision, the higher the more precise,//
                (x, {                                                   //can break if (-2500000..=2500000) are not changed//
                    let p1 = 3.5_f64 / ((2.5 * PI).sqrt());
                    let p2 = (magic_level + 2.0).sqrt();
                    let p3 = p1 * p2;
                    let p4 = -(x).powf(2.0);
                    let p5 = p4 / (2.5 * (magic_level + 2.0)); //FG(x) = ((3.5/sqrt(2.5pi)* sqrt(n -2))* e)^ -(x-x0)^2/2.5(n-2)//
                    (E.powf(p5)) * p3 //calculations, this was a pain to set up lmao//
                    })
                }),
                &RED,
        ))?
        .label("Magic Missile GF")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

        chart
            .configure_series_labels()
            .background_style(&WHITE.mix(0.8))
            .border_style(&BLACK)
            .draw()?; //draws the actual graph//
    }
    }
}
