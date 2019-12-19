fn main() {
  let input = include_str!("input.txt");

  /*
  Images are sent as a series of digits that each represent the color of a single pixel. The digits fill each row of the image left-to-right, then move downward to the next row, filling rows top-to-bottom until every pixel of the image is filled.

  Each image actually consists of a series of identically-sized layers that are filled in this way. So, the first digit corresponds to the top-left pixel of the first layer, the second digit corresponds to the pixel to the right of that on the same layer, and so on until the last digit, which corresponds to the bottom-right pixel of the last layer.

  The image you received is 25 pixels wide and 6 pixels tall.

  To make sure the image wasn't corrupted during transmission, the Elves would like you to find the layer that contains the fewest 0 digits. On that layer, what is the number of 1 digits multiplied by the number of 2 digits?
  */
  let width = 25;
  let height = 6;
  let data: Vec<_> = input
    .chars()
    .filter_map(|c| c.to_string().parse().ok())
    .collect();

  let layers: Vec<_> = data.chunks(width * height).collect();

  let min_0s_layer = layers
    .iter()
    .min_by_key(|l| l.iter().filter(|d| **d == 0).count())
    .unwrap();
  let (ones, twos) = min_0s_layer.iter().fold((0, 0), |(ones, twos), d| match d {
    1 => (ones + 1, twos),
    2 => (ones, twos + 1),
    _ => (ones, twos),
  });
  println!("Part 1: {}", ones * twos); // 1340

  /*
  Now you're ready to decode the image. The image is rendered by stacking the layers and aligning the pixels with the same positions in each layer. The digits indicate the color of the corresponding pixel: 0 is black, 1 is white, and 2 is transparent.

  The layers are rendered with the first layer in front and the last layer in back. So, if a given position has a transparent pixel in the first and second layers, a black pixel in the third layer, and a white pixel in the fourth layer, the final image would have a black pixel at that position.

  Then, the full image can be found by determining the top visible pixel in each position

  What message is produced after decoding your image?
  */
  println!("Part 2:");
  for y in 0..height {
    for x in 0..width {
      for l in &layers {
        match l[y * width + x] {
          0 => {
            print!(" ");
            break;
          }
          1 => {
            print!("#");
            break;
          }
          _ => (),
        };
      }
    }
    print!("\n");
  }
  /*
  #    ####   ## #  #  ##
  #    #       # # #  #  #
  #    ###     # ##   #
  #    #       # # #  #
  #    #    #  # # #  #  #
  #### ####  ##  #  #  ##
  */
}
