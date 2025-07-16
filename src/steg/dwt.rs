use crate::{save_luma8_image, steg::Colour, HEIGHT, WIDTH};
fn rows<'a>(image: &'a [u8]) -> impl Iterator<Item=&'a [u8]> {
    return image.chunks(WIDTH)
}

fn cols(image: &[u8]) -> impl Iterator<Item=Vec<u8>>  {
    (0..WIDTH)
    .map(|column| {
        image
        .iter()
        .skip(column)
        .step_by(WIDTH)
        .copied()
        .collect::<Vec<u8>>()
    })
    .collect::<Vec<_>>()
    .into_iter()
}


fn cols_to_img(cols: &[u8]) -> Vec<u8> {
    let mut ret = vec![0u8; WIDTH * HEIGHT];
    for (idx, &pixel) in cols.iter().enumerate() {
        let x = idx as usize / HEIGHT;
        let y = idx as usize % HEIGHT;
        ret[y*WIDTH+x] = pixel
    }
    ret
}



fn haar_dwt(bit_plane: &[u8], col: Colour) {
    let mut row_buf = vec![];
    let mut col_buf = vec![];

    for row in rows(bit_plane) {
        let mut lhs: Vec<u8> = vec![];
        let mut rhs: Vec<u8> = vec![];
        for pair in row.chunks(2) {
            let p1 = pair[0];
            let p2 = pair[1];
            lhs.push(p1.saturating_add(p2));
            rhs.push(p1.saturating_sub(p2));
        }
        row_buf.extend(lhs);
        row_buf.extend(rhs);
    }

    for col in cols(&row_buf) {
        let mut higher: Vec<u8> = vec![];
        let mut lower: Vec<u8> = vec![];
        for pair in col.chunks(2) {
            let p1 = pair[0];
            let p2 = pair[1];
            higher.push(p1.saturating_add(p2));
            lower.push(p1.saturating_sub(p2));
        }
        col_buf.extend(higher);
        col_buf.extend(lower);
    }

    let image = cols_to_img(&col_buf);

    save_luma8_image(&image, format!("grayTest{:?}", col));
    
}

pub fn embed(image: &[[u8; 4]]) -> (usize, usize) {
    // first unzip into Vec<r> and Vec<(g,b)>
    let (rs, gb): (Vec<u8>, Vec<(u8,u8)>) = image
        .iter()
        .map(|&[r,g,b,_]| (r, (g,b)))
        .unzip();

    // then unzip that Vec<(g,b)> into Vec<g> and Vec<b>
    let (gs, bs): (Vec<u8>, Vec<u8>) = gb.into_iter().unzip();

    haar_dwt(&rs, Colour::RED);
    haar_dwt(&gs, Colour::GREEN);
    haar_dwt(&bs, Colour::BLUE);


    //haar_dwt(&image_to_luma8(&image));
    (0,0)
}

pub fn solve() -> String {


    String::from("Boo")
}