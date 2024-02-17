import "lib/github.com/diku-dk/sorts/merge_sort"

def binarySearch [n] 't (lte: t -> bool) (xs: [n]t) : i64 =
  let (l, _) =
    loop (l, r) = (0, n-1) while l < r do
    let t = l + (r - l) / 2
    in if lte xs[t]
       then (l, t)
       else (t+1, r)
  in l

def rangeSearch [n] (low: f32) (high: f32) (xs: [n]f32) : (i64, i64) =
  let (l, _, m) =
    loop (l, r, m) = (0, n-1, n-1) while l < r do
    let t = l + (r - l) / 2
    in if low <= xs[t]
       then (l, t, if xs[t] >= high then t else m)
       else (t+1, r, m)
  let (h, _) = 
    loop (l, r) = (l, m) while l < r do
    let t = l + (r - l) / 2
    in if high <= xs[t]
       then (l, t)
       else (t+1, r)
  in (l, h)
  

def originalForce (d: f32) (coupling: f32) (dx: f32) (dy: f32): (f32, f32) = 
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

def cheapForce (d: f32) (coupling: f32) (dx: f32) (dy: f32): (f32, f32) = 
    let exclusion = 0.05 / (d*d) - 0.05
    let attraction = coupling * (1 - d)
    let force = attraction - exclusion
    in (force * dx / d, force * dy / d)

-- terms reorganized in an incomprehensible way to only do one division instead of three
-- see https://www.wolframalpha.com/input?i=%28c%281-x%29+-+%28b%2Fx%5E2+-+b%29%29%2Fx
def cheapForceSimple (d: f32) (coupling: f32) (dx: f32) (dy: f32): (f32, f32) = 
    let d2 = d*d
    let d3 = d2*d
    let forceperd = (0.05 * d2 - 0.05 + d2 * coupling * (1-d))/d3
    in (forceperd * dx, forceperd * dy)

def signedSquare (x: f32): f32 = if x < 0 then -x*x else x*x

def clamp (x: f32) (a: f32) (b: f32): f32 = 
  let min = if a < b then a else b
  let max = if a < b then b else a
  in if x < min then min else if x > max then max else x

-- This is a simple quadratic drag model. Real drag is linear at low speeds.
-- To change this to a simple _friction_ model instead of a drag model, just make
-- this to `v - 0.05 * v`.
def applyDrag (v: f32): f32 = if f32.abs v < 1.0 then v - 0.05 * v else v - clamp (0.05 * signedSquare(v)) 0 (v)

def step_body [n][m] 
    (forces: [m][m]f32) 
    (particles: [n](i8, f32, f32, f32, f32))
    (dt: f32):
    [n](i8, f32, f32, f32, f32)
    = map (\(t, vx, vy, px, py) -> 
      let (start, end) = rangeSearch (py-1.0) (py+1.0) (map (\(_, _, _, _, y) -> y) particles)
        let (fx, fy) = reduce_comm (\(ax, ay) (bx, by) -> (ax + bx, ay + by)) (0.0,0.0) (
            map (\(ot, _, _, ox, oy) -> if ox == px && oy == py then (0.0, 0.0) else -- don't apply force to self
                let dx = ox - px
                let dy = oy - py
                let sqd = dx*dx + dy*dy
                in if sqd > 1.0 then (0.0, 0.0) else
                    let d = f32.sqrt sqd
                    let coupling = forces[t][ot]
                    in cheapForceSimple d coupling dx dy
            ) particles[start:end])
        in (t, applyDrag(vx + fx * dt), applyDrag(vy + fy * dt), px + vx * dt, py + vy * dt)
    ) particles

entry step [n][m] 
    (forces: [m][m]f32) 
    (types: [n]i8)
    (velocity_x: [n]f32)
    (velocity_y: [n]f32)
    (position_x: [n]f32)
    (position_y: [n]f32)
    (dt: f32):
    ([n]i8, [n]f32, [n]f32, [n]f32, [n]f32) =
    let particles = zip5 types velocity_x velocity_y position_x position_y
    let particles' = merge_sort_by_key (\(_, _, _, _, y) -> y) (\a b -> a <= b) particles
    in unzip5 (step_body forces particles' dt)