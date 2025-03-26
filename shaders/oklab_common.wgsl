// cube root
fn cbrt(x: f32) -> f32 {
    return pow(x, 1.0 / 3.0);
}
const pi: f32 =
    3.1415926535897932384626433832795028841971693993751058209749445923078164062f;

// https://bottosson.github.io/posts/colorpicker/
struct Lab { L: f32, a: f32, b: f32 };
struct RGB { r: f32, g: f32, b: f32 };
struct HSV { h: f32, s: f32, v: f32 };
struct LC { L: f32, C: f32 };
struct ST { S: f32, T: f32 };
fn toe(x: f32) -> f32 {
  var k_1: f32 = 0.206f;
  var k_2: f32 = 0.03f;
  var k_3: f32 = (1.f + k_1) / (1.f + k_2);
  return 0.5f * (k_3 * x - k_1 +
                 sqrt((k_3 * x - k_1) * (k_3 * x - k_1) + 4 * k_2 * k_3 * x));
}
fn toe_inv(x: f32) -> f32 {
  var k_1: f32 = 0.206f;
  var k_2: f32 = 0.03f;
  var k_3: f32 = (1.f + k_1) / (1.f + k_2);
  return (x * x + k_1 * x) / (k_3 * (x + k_2));
}
fn to_ST(cusp: LC) -> ST {
  var L: f32 = cusp.L;
  var C: f32 = cusp.C;
  return ST(C / L, C / (1 - L));
}
fn oklab_to_linear_srgb(c: Lab) -> RGB {
  var l_: f32 = c.L + 0.3963377774f * c.a + 0.2158037573f * c.b;
  var m_: f32 = c.L - 0.1055613458f * c.a - 0.0638541728f * c.b;
  var s_: f32 = c.L - 0.0894841775f * c.a - 1.2914855480f * c.b;

  var l: f32 = l_ * l_ * l_;
  var m: f32 = m_ * m_ * m_;
  var s: f32 = s_ * s_ * s_;

  return RGB(
       4.0767416621f * l - 3.3077115913f * m + 0.2309699292f * s,
      -1.2684380046f * l + 2.6097574011f * m - 0.3413193965f * s,
      -0.0041960863f * l - 0.7034186147f * m + 1.7076147010f * s,
  );
}
fn compute_max_saturation(a: f32, b: f32) -> f32 {
  // Max saturation will be when one of r, g or b goes below zero.

  // Select different coefficients depending on which component goes below zero
  // first
  var k0: f32;
  var k1: f32;
  var k2: f32;
  var k3: f32;
  var k4: f32;
  var wl: f32;
  var wm: f32;
  var ws: f32;

  if (-1.88170328f * a - 0.80936493f * b > 1) {
    // Red component
    k0 = 1.19086277f;
    k1 = 1.76576728f;
    k2 = 0.59662641f;
    k3 = 0.75515197f;
    k4 = 0.56771245f;
    wl = 4.0767416621f;
    wm = -3.3077115913f;
    ws = 0.2309699292f;
  } else if (1.81444104f * a - 1.19445276f * b > 1) {
    // Green component
    k0 = 0.73956515f;
    k1 = -0.45954404f;
    k2 = 0.08285427f;
    k3 = 0.12541070f;
    k4 = 0.14503204f;
    wl = -1.2684380046f;
    wm = 2.6097574011f;
    ws = -0.3413193965f;
  } else {
    // Blue component
    k0 = 1.35733652f;
    k1 = -0.00915799f;
    k2 = -1.15130210f;
    k3 = -0.50559606f;
    k4 = 0.00692167f;
    wl = -0.0041960863f;
    wm = -0.7034186147f;
    ws = 1.7076147010f;
  }

  // Approximate max saturation using a polynomial:
  var S: f32 = k0 + k1 * a + k2 * b + k3 * a * a + k4 * a * b;

  // Do one step Halley's method to get closer
  // this gives an error less than 10e6, except for some blue hues where the
  // dS/dh is close to infinite this should be sufficient for most applications,
  // otherwise do two/three steps

  var k_l: f32 =  0.3963377774f * a + 0.2158037573f * b;
  var k_m: f32 = -0.1055613458f * a - 0.0638541728f * b;
  var k_s: f32 = -0.0894841775f * a - 1.2914855480f * b;

  {
    var l_: f32 = 1.f + S * k_l;
    var m_: f32 = 1.f + S * k_m;
    var s_: f32 = 1.f + S * k_s;

    var l: f32 = l_ * l_ * l_;
    var m: f32 = m_ * m_ * m_;
    var s: f32 = s_ * s_ * s_;

    var l_dS: f32 = 3.f * k_l * l_ * l_;
    var m_dS: f32 = 3.f * k_m * m_ * m_;
    var s_dS: f32 = 3.f * k_s * s_ * s_;

    var l_dS2: f32 = 6.f * k_l * k_l * l_;
    var m_dS2: f32 = 6.f * k_m * k_m * m_;
    var s_dS2: f32 = 6.f * k_s * k_s * s_;

    var f: f32 = wl * l + wm * m + ws * s;
    var f1: f32 = wl * l_dS + wm * m_dS + ws * s_dS;
    var f2: f32 = wl * l_dS2 + wm * m_dS2 + ws * s_dS2;

    S = S - f * f1 / (f1 * f1 - 0.5f * f * f2);
  }

  return S;
}
fn srgb_transfer_function_inv(a: f32) -> f32 {
  if .04045f < a {
    return pow((a + .055f) / 1.055f, 2.4f);
  } else {
    return a / 12.92f;
  }
}
fn linear_srgb_to_oklab(c: RGB) -> Lab {
  var l: f32 = 0.4122214708f * c.r + 0.5363325363f * c.g + 0.0514459929f * c.b;
  var m: f32 = 0.2119034982f * c.r + 0.6806995451f * c.g + 0.1073969566f * c.b;
  var s: f32 = 0.0883024619f * c.r + 0.2817188376f * c.g + 0.6299787005f * c.b;

  var l_: f32 = cbrt(l);
  var m_: f32 = cbrt(m);
  var s_: f32 = cbrt(s);

  return Lab(
      0.2104542553f * l_ + 0.7936177850f * m_ - 0.0040720468f * s_,
      1.9779984951f * l_ - 2.4285922050f * m_ + 0.4505937099f * s_,
      0.0259040371f * l_ + 0.7827717662f * m_ - 0.8086757660f * s_,
  );
}
fn find_cusp(a: f32, b: f32) -> LC {
  // First, find the maximum saturation (saturation S = C/L)
  var S_cusp: f32 = compute_max_saturation(a, b);

  // Convert to linear sRGB to find the first point where at least one of r,g or
  // b >= 1:
  var rgb_at_max: RGB = oklab_to_linear_srgb(Lab(1, S_cusp * a, S_cusp * b));
  var L_cusp: f32 =
    cbrt(1.0 / max(max(rgb_at_max.r, rgb_at_max.g), rgb_at_max.b));
  var C_cusp: f32 = L_cusp * S_cusp;

  return LC(L_cusp, C_cusp);
}
fn srgb_to_okhsv(rgb: RGB) -> HSV
{
	var lab: Lab = linear_srgb_to_oklab(RGB(
		srgb_transfer_function_inv(rgb.r),
		srgb_transfer_function_inv(rgb.g),
		srgb_transfer_function_inv(rgb.b)
    ));

	var C: f32 = sqrt(lab.a * lab.a + lab.b * lab.b);
	var a_: f32 = lab.a / C;
	var b_: f32 = lab.b / C;

	var L: f32 = lab.L;
	var h: f32 = 0.5f + 0.5f * atan2(-lab.b, -lab.a) / pi;

	var cusp: LC = find_cusp(a_, b_);
	var ST_max: ST = to_ST(cusp);
	var S_max: f32 = ST_max.S;
	var T_max: f32 = ST_max.T;
	var S_0: f32 = 0.5f;
	var k: f32 = 1 - S_0 / S_max;

	// first we find L_v, C_v, L_vt and C_vt

	var t: f32 = T_max / (C + L * T_max);
	var L_v: f32 = t * L;
	var C_v: f32 = t * C;

	var L_vt: f32 = toe_inv(L_v);
	var C_vt: f32 = C_v * L_vt / L_v;

	// we can then use these to invert the step that compensates for the toe and the curved top part of the triangle:
	var rgb_scale: RGB = oklab_to_linear_srgb(Lab(L_vt, a_ * C_vt, b_ * C_vt));
	var scale_L: f32 = cbrt(1.f / max(max(rgb_scale.r, rgb_scale.g), max(rgb_scale.b, 0.f)));

	L = L / scale_L;
	C = C / scale_L;

	C = C * toe(L) / L;
	L = toe(L);

	// we can now compute v and s:

	var v: f32 = L / L_v;
	var s: f32 = (S_0 + T_max) * C_v / ((T_max * S_0) + T_max * k * C_v);

	return HSV(h, s, v);
}

