# Workbook

## 07-11-26, Researching elementary function implementations

Today I spent time searching for a good manual on implementing elementary functions and eventually found "Cody & Waite, Software Manual for the Elementary Functions" as something that was available to download. My hope is that this contains the formulas for most functions needed such as ln and exp to where I can implement them on hardware, along with potentially having other tips for writing math in low level code. Alongside that, I tried a 1 line change to the traverse function to snap the position to a boundary when it crosses a border, however I didn't have time to test it as the research took most of the time today.

## 07-10-26, -ln(ε) optimizations continued, debugging

### -ln
Looking back over, there are some more things that could be a bit better with the optimizations. First, `k` in $ k ln(2) $ is a pretty small integer, and $ ln(2) $ is a constant, meaning that it probably shouldn't be in its own LUT. Second, I was thinking about trying to do use a set of slopes instead of just storing a LUT. From googling I believe that's called a piece wise approximation, and I think that it would be good to increase some accuracy at the cost of a bit of compute, although I need to do more research on it.

One thing that felt like it could be optimized is how the piece wise function is distributed. If each slope is just the derivative of ln at certain points (f'), and each derivative is distributed based on the change of derivative (f''), there is probably some way to optimize using this correlation, however this is probably just a mathematical dead end.

### Debugging

Additionally, I worked on some debugging today in the f32 reference now that the scripts are working. First, I found that inv_dir can be equal to +-inf in voxel_t_exit whenever a direction is 0 as NaN is passed into partial_cmp, causing a panic on unwrap. To fix that I just changed it to a manual for loop where I discard any dir equal to exactly 0.0. Second, just to ensure that in the case of getting a path length of 0.0 (when random generates 1.0), I rewrote the random code to just reroll until eps isn't equal to 0 or 1, avoiding any annoying rng semantics. Next, I added an outer loop label on the loop in resolve photon to allow it to properly break. Finally, I changed the `kept` fraction to be calculated using exp_m1 over exp, as when I was researching what .exp actually did I found that .exp_m1 is much better for small numbers.

While not something that was a full error that came up, I can see the position update in traverse being an issue. For example, if a photon traverses from a voxel to another voxel to the right, (in other words on the x axis traveling x+) and ends up in a new voxel, it is assumed that its position in the x-axis is now wrapped back to 0.0. However, if there's any error at all in the fixed point, then this could be a few bits lower and end up being much closer to 0.999, completely breaking the position code. As I write this, this seems like a simulation breaking error I should work on fixing. All I would have to do is just set the position on the traveled axis to 0 or 1 depending on the sign of the direction.

## 07-09-26, -ln(ε) optimizations
Today a spent some time researching variance and bias as I'm interested in their application in the final HLS implementation. From my understanding variance is just a random addition to the output while bias is when that variance is biased. The whole point of SR is that it gets rid of bias in place of variance, and since monte carlo is random, when a system is in place for variance to be traded for speed can be heavily abused. For example with the path sampling function, $ - ln(\varepsilon) $, it can be approximated as $ -(ln(m) - k ln(2)) = kln(2) - ln(m) $ since $ \varepsilon \in (0, 1) $. What I need to do research on is how the bias travels through LUTS.

For example, $ ln(m) $ where $ m \in [1, 2) $ will cause some bias. My idea to fix that would be to use a global LUT for values between 1 and 2, maybe having each store an $ m $ and $ b $ value (from $ y = mx + b $) over tiny segments. I could make this LUT pretty big as my BRAM is completely unused by large storage yet (caches are in URAM as of now), and say all 10 concurrent $ - ln $ kernels could share the one LUT.

That would still cause some bias but a lot less than just running $ - ln(\varepsilon) $, and of course the same large LUT applies to $ kln(2) $.

Branching off of the idea of using BRAM for a LUT, all of my BRAM should be basically empty, only being really used by like local variables. I have about 0.64 MB of BRAM, so I could use basically all of that for LUTS instead of using actual logic LUTS. That could be good for a later implementation if I want to lean on LUTS the more I think about it.