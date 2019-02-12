extern crate gnuplot;
use gnuplot::*;




fn main(){
    let spacing = 99.84; // MHz
    //let center_freq = i32 27900; // MHz for sc_pos=[-3308, -2476, -1644, -812, 20, 852, 1684, 2516];
    let center_freq = 27900.0+3.84; // MHz for sc_pos=[ -3296, -2464, -1632, -800, 32, 864, 1696, 2528]; I can't explain why there is a 3.84MHz difference.
    let num_cc = 8_usize; // number of components
    let steps_inside_cc = 20.0; // in MHz

    //Check step size
    assert_eq!(steps_inside_cc > 99.84/2.0, false);

    let start_freq = center_freq - (num_cc as f64 / 2.0 * spacing); // Lowest frequency of in-band.
    let num_steps_inside = ( spacing / steps_inside_cc ).floor() as usize;
    
    let mut results = vec![0_f64; (num_cc*num_steps_inside) as usize];
    let mut center_cc = vec![0_f64; num_cc];
    let mut index = 0;
    let mut step: f64;

    for cc in 0..num_cc {
        center_cc[cc] = start_freq + (cc as f64 * spacing) + spacing/2.0; // center frequency of each component
        for num_step in 0..num_steps_inside {
            //index += index;
            step = num_step as f64 - ((num_steps_inside as f64 - 1.0) / 2.0); // center so fx. for num_step = 5 -> -2,-1,0,1,2, or num_step = 6 -> -2.5,-1.5,-0.5,0.5,1.5,2,5
            results[index] = center_cc[cc] + step * steps_inside_cc as f64;
            index = index + 1;
        }
    }
    dbg!(&results);

    let y = vec![10.0; results.len()];
    let channel_bw = vec![95.04; results.len()]; // MHz

    let mut fig = Figure::new();
    fig.axes2d()
        //.lines(&results, &y, &[Caption("Test Frequencies"), Color("blue")]);
        .points(&results, &y, &[PointSymbol('o'), Color("blue") ])
        .boxes_set_width(&center_cc, &y, &channel_bw, &[Color("blue")]);
    fig.show();

/*
    let x = [0u32, 1, 2];
    let y = [3u32, 4, 5];
    let mut fg = Figure::new();
    fg.axes2d()
        .lines(&x, &y, &[Caption("A line"), Color("black")]);
    fg.show();
*/

}
