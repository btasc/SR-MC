

# 07-09-26, -ln(ε) optimizations
Today a spent some time researching variance and bias as I'm interested in their application in the final HLS implementation. From my understanding variance is just a random addition to the output while bias is when that variance is biased. The whole point of SR is that it gets rid of bias in place of variance, and since monte carlo is random, when a system is in place for variance to be traded for speed can be heavily abused. For example with the path sampling function, $ - ln(\varepsilon) $, it can be approximated as $ -(ln(m) - k ln(2)) = kln(2) - ln(m) $ since $ \varepsilon \in (0, 1) $. What I need to do research on is how the bias travels through LUTS.

For example, $ ln(m) $ where $ m \in [1, 2) $ will cause some bias. My idea to fix that would be to use a global LUT for values between 1 and 2, maybe having each store an $ m $ and $ b $ value (from $ y = mx + b $) over tiny segments. I could make this LUT pretty big as my BRAM is completely unused by large storage yet (caches are in URAM as of now), and say all 10 concurrent $ - ln $ kernels could share the one LUT.

That would still cause some bias but a lot less than just running $ - ln(\varepsilon) $, and of course the same large LUT applies to $ kln(2) $.

Branching off of the idea of using BRAM for a LUT, all of my BRAM should be basically empty, only being really used by like local variables. I have about 0.64 MB of BRAM, so I could use basically all of that for LUTS instead of using actual logic LUTS. That could be good for a later implementation if I want to lean on LUTS the more I think about it.