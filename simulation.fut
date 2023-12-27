def simpleForce (d: f32) (coupling: f32) (dx: f32) (dy: f32): (f32, f32) = 
    -- This following code uses a simple triangular model:
    --               __a___
    --           __--      ---___
    --       __--                ---___
    --   --e----------------------------e------
    --    /
    --   /
    -- We have an exclusion force that pushes all nearby particles away;
    -- the excluson force and attraction force balance at `exclusion` distance,
    -- no matter the strength of the attraction force.
    -- Then we linearly increase until the `attraction` distance, which is the
    -- peak of the model. The height of this is determined by the coupling distance.
    -- Finally, we have a linear falloff until the `edge` distance, which simulates
    -- forces going to zero at infinity.
    -- I don't love this model but I saw it in a youtube video so who knows.
    -- One of the reasons this model is sus is that for repelling couping forces, 
    -- there's still a point where the exclusion force and the coupling force "balance" to
    -- zero, despite them both being negative!
    let exclusion = 0.1
    let attraction = 0.5
    let edge = 1.0
    let force = if d < exclusion then (1/exclusion) * (d - 1)
        else if d < attraction then (d-exclusion)*coupling
        else if d < edge then coupling*(attraction-exclusion)*(edge-d)/(edge-attraction)
        else 0
    in (force * dx / d, force * dy / d)

def fancyForce (d: f32) (coupling: f32) (dx: f32) (dy: f32): (f32, f32) = 
    -- this is basically inspired by van-der-waal's forces
    -- it uses exponents 12 and 6. Interesting
    -- to see how the behavior would change as those exponents change.
    -- Also, this could be made more efficient by removing the division by
    -- d in the final line and increasing the multipliers by 1. Possibly
    -- the compiler can optimize that but if it can't, we should.
    -- RIGHT NOW THIS BLOWS UP TO INFINITY oops.
    let exclusionCoupling = -0.000001
    let d2 = d*d
    let d4 = d2*d2
    let d6 = d4*d2
    let d12 = d6*d6
    let f = (coupling/d6 + exclusionCoupling/d12)
    in ((dx/d) * f, (dy/d) * f)

def sigm (x: f32): f32 = 1/(1+ f32.exp (-x))

def signedSquare (x: f32): f32 = if x < 0 then -x*x else x*x

def clamp (x: f32) (a: f32) (b: f32): f32 = 
  let min = if a < b then a else b
  let max = if a < b then b else a
  in if x < min then min else if x > max then max else x

-- This is a simple quadratic drag model. Real drag is linear at low speeds.
-- To change this to a simple _friction_ model instead of a drag model, just make
-- this to `v - 0.05 * v`.
def applyDrag (v: f32): f32 = v - clamp (0.05 * signedSquare(v)) 0 (v)

entry step [n][m] 
    (forces: [m][m]f32) 
    (types: [n]i8)
    (velocity_x: [n]f32)
    (velocity_y: [n]f32)
    (position_x: [n]f32)
    (position_y: [n]f32)
    (dt: f32):
    ([n]i8, [n]f32, [n]f32, [n]f32, [n]f32) = let particles = zip5 types velocity_x velocity_y position_x position_y in 
    unzip5 (map (\(t, vx, vy, px, py) -> 
        let (fx, fy) = reduce (\(ax, ay) (bx, by) -> (ax + bx, ay + by)) (0.0,0.0) (
            map (\(ot, _, _, ox, oy) -> if ox == px && oy == py then (0.0, 0.0) else -- don't apply force to self
                let dx = ox - px
                let dy = oy - py
                let d = f32.hypot dx dy -- hypot is an optimized & accurate sqrt(dx*dx + dy*dy)
                let coupling = forces[t][ot]
                in simpleForce d coupling dx dy
            ) particles)
        in (t, applyDrag(vx + fx * dt), applyDrag(vy + fy * dt), px + vx * dt, py + vy * dt)
    ) particles)



-- global values for the benchmark

def randoms =
    [0.39694452f32, 0.440077f32, 4.184246e-4f32, 0.33152735f32, 0.5394453f32, 0.58913165f32, 0.14590704f32, 0.8691784f32, 0.35399735f32, 0.52074766f32, 0.19089913f32, 0.60961545f32, 0.6844547f32, 0.99549663f32, 0.67218816f32, 0.5143038f32, 0.59208596f32, 0.39684755f32, 0.5641243f32, 0.6494475f32, 8.409703e-2f32, 0.68260133f32, 0.8933851f32, 0.7557137f32, 6.795347e-2f32, 1.1960506e-2f32, 0.47301173f32, 2.9440701e-2f32, 0.12855893f32, 0.22780854f32, 0.3079248f32, 0.21531892f32, 0.5856429f32, 0.47521514f32, 0.33820915f32, 0.5510872f32, 0.35372198f32, 0.9136436f32, 9.9897504e-2f32, 0.67588294f32, 0.2088086f32, 0.4339999f32, 0.9875505f32, 0.15360159f32, 0.21731955f32, 0.5188843f32, 0.72519785f32, 0.27903384f32, 0.55019724f32, 0.8123876f32, 0.32590687f32, 0.83556914f32, 0.85068285f32, 0.36308002f32, 0.7723786f32, 0.77875865f32, 0.86058426f32, 0.5013354f32, 0.8299845f32, 0.18427992f32, 0.19694507f32, 0.84816813f32, 4.0673733e-2f32, 0.5715263f32, 0.60798f32, 0.6171998f32, 0.7213495f32, 0.49193698f32, 6.651425e-2f32, 0.79721946f32, 0.18684405f32, 0.57188785f32, 0.93566304f32, 0.2386474f32, 0.9367501f32, 0.27438807f32, 0.52753985f32, 0.7757174f32, 0.84410936f32, 4.0887296e-2f32, 0.7217814f32, 0.8356373f32, 0.7391924f32, 0.33688766f32, 0.26087815f32, 9.017104e-2f32, 0.124531984f32, 0.67375165f32, 7.90233e-2f32, 0.3666563f32, 0.6285253f32, 0.71400243f32, 0.7922644f32, 0.40123427f32, 0.7058128f32, 0.21938539f32, 0.4102047f32, 0.69586676f32, 0.24294418f32, 0.2699461f32, 2.3607075e-2f32, 0.5883666f32, 0.64184654f32, 0.458979f32, 0.123099804f32, 0.7963545f32, 0.8499712f32, 0.8688344f32, 0.35088933f32, 0.6018281f32, 0.8317536f32, 0.16279691f32, 0.46765387f32, 2.355647e-2f32, 0.9335027f32, 0.5191835f32, 0.32872486f32, 0.5871488f32, 0.7437369f32, 0.92296994f32, 0.810787f32, 0.7092232f32, 0.3360734f32, 0.52803457f32, 0.20817894f32, 0.69732815f32, 0.40970528f32, 0.92471987f32, 0.4013633f32, 0.5257565f32, 0.11624676f32, 0.425314f32, 0.9942921f32, 0.5302127f32, 0.36761153f32, 0.5430516f32, 0.42867255f32, 0.70089465f32, 0.8980731f32, 0.4098121f32, 0.4511497f32, 0.9509155f32, 0.42013645f32, 6.714201e-2f32, 0.7475324f32, 0.82486963f32, 0.21372879f32, 3.450036e-2f32, 0.664199f32, 0.5520524f32, 0.5659759f32, 0.19425946f32, 0.64716804f32, 0.62561774f32, 0.18178475f32, 0.4123934f32, 0.62190586f32, 0.6303042f32, 0.9025019f32, 0.5308182f32, 1.8090367e-2f32, 0.7493145f32, 0.25189024f32, 0.6590699f32, 0.2669974f32, 0.5617765f32, 0.11337918f32, 0.8710372f32, 0.3999198f32, 0.7465745f32, 0.8535186f32, 0.4388169f32, 0.61773294f32, 0.5474007f32, 0.4989341f32, 0.19402158f32, 7.4068904e-2f32, 0.5041898f32, 0.3943231f32, 0.7245204f32, 0.15002799f32, 0.3422473f32, 0.2863313f32, 0.4169684f32, 0.81865066f32, 0.23959595f32, 0.49010712f32, 0.99748075f32, 0.93584204f32, 4.064125e-2f32, 0.30564284f32, 2.4187148e-2f32, 0.15391862f32, 0.74308884f32, 0.7229351f32, 0.3270734f32, 0.92789125f32, 0.39614403f32, 0.24116796f32, 0.54730105f32]
def forces: [3][3]f32 =
    [[0.39694452f32, -0.440077f32, 4.184246e-4f32],
    [-0.33152735f32, -0.5394453f32, 0.58913165f32],
    [0.14590704f32, -0.8691784f32, -0.35399735f32]]   
def init: [1000](i8, f32, f32, f32, f32) = zip5 
    ((replicate 340 0 ++ replicate 330 1 ++ replicate 330 2) :> [1000]i8)
    (replicate 1000 0.0)
    (replicate 1000 0.0)
    (rotate 50 (randoms ++ reverse randoms ++ randoms ++ reverse randoms ++ randoms) :> [1000]f32)
    (rotate 777 (reverse randoms ++ randoms ++ reverse randoms ++ randoms ++ randoms) :> [1000]f32)

-- Benchmark how long step takes
-- ==
-- entry: bench_step
-- input { 200 }
-- auto output

entry bench_step (iters: i32): ([1000]i8, [1000]f32, [1000]f32, [1000]f32, [1000]f32) =
    let f = copy forces
    let i = copy init
    in loop (types, velocity_x, velocity_y, position_x, position_y) = unzip5 i for _i < iters do
        step f types velocity_x velocity_y position_x position_y 0.01
