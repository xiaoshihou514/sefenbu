struct HSL { h: f32, s: f32, l: f32 };
fn srgb_to_hsl(r: f32, g: f32, b: f32) -> HSL {
    var max: f32 = max(max(r, g), b);
    var min: f32 = min(min(r, g), b);
    var h: f32 = (max + min) / 2;
    var s: f32 = (max + min) / 2;
    var l: f32 = (max + min) / 2;

    if (max == min) {
        h = 0.;
        s = 0.;
    } else {
        var d = max - min;
        if l > 0.5 { s = d / (2 - max - min); } else { s = d / (max + min); }

        if max == r {
            h = (g - b) / d;
            if g < b {
                h = h + 6;
            }
        } else if max == g {
            h = (b - r) / d + 2;
        } else if max == b {
            h = (r - g) / d + 4;
        }

        h = h / 6;
    }

    return HSL(h, s, l);
}

fn hue2rgb(p: f32, q: f32, t_: f32) -> f32 {
    var t = t_;
    t = fract(t + 1.0);  // => t +=1
    if (t < 1.0/6.0) {
        return p + (q - p) * 6.0 * t;
    } else if (t < 1.0/2.0) {
        return q;
    } else if (t < 2.0/3.0) {
        return p + (q - p) * (2.0/3.0 - t) * 6.0;
    }
    return p;
}

fn hsl_to_srgb(h: f32, s: f32, l: f32) -> vec3<f32> {
    if (s == 0.0) {
        return vec3<f32>(l, l, l);
    }

    let q = select(l + s - l * s, l * (1.0 + s), l < 0.5);
    let p = 2.0 * l - q;
    
    let r = hue2rgb(p, q, fract(h + 1.0/3.0));
    let g = hue2rgb(p, q, fract(h));
    let b = hue2rgb(p, q, fract(h - 1.0/3.0));

    return vec3<f32>(r, g ,b);
}
