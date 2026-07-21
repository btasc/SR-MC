# Workbook

## 07-20-26, Rewrote rationale, worked on bibliography, ideas for bit sweep

Today I worked on rewriting the rationale. Originally I had thought that the Rationale was for the reasoning on why the technique would work, but it seems to more be the actual use of the technique. I've spent a lot of time working on it and editing it, however the current version is very long. I believe tomorrow I would like to edit it down, removing much of the unnecessary technical explanations and such, where I can then finally finish my forms and submit them.

I attempted to work on the bibliography. I downloaded the citations manager Zotero, however have not spent much time using it, as I dedicated most of my time today on my Rationale and finding the relevant citations for it.

Finally, I had some time to think about how I want to implement the bit sweep. I think the best way to do it would be to use some struct that gives a "pool" of ap nums to use. whenever a new bit sweep variable is made, a number can be fetched with something like `pool.get_ap::<ApInt>()` or something, then an operation can be done with something like `pool.mul::<ApInt>(n1, n2)` or `pool.lut::<ApInt>(lut, n1)`. To bit sweep, there can be a bitsweep method that passes in a `SweepPool` to the chosen function. It can be run in a no operation mode where it only adds a new value to the pool, then once all the "to-be-swept" values are added, it can run through and change each one's width. This seems like the best method. I also want to change my ApInt to have a max width of like 60, as right now there's some annoying signed semantics happening where I want to avoid using a max of 63. I also need to add a guard that avoids overflow, as right now I think the multiply check doesn't work on overflow.

## 07-19-26, Finalizing research plan, starting bibliography tomorrow

Today I worked on my research plan more, rewriting sections and trying to get it closer to a final version. I've written my core sections to a point where I think they are ready, besides my rationale section, however now I need to start on my bibliography formatting. From my understanding, ISEF allows any format of citations, and LVSF / DVSF default to ISEF, meaning I get to pick the format. From my research, IEEE format seems to be correct for my project, allowing me to write short in text citations in the form of [n], [n, m, o], and [n-z], which I enjoy for writing and have enjoyed in the research papers I've read. For now however, my in text citations are just in the form of links.

I plan to finish the citations, do one final review of my research plan, and finish the forms all tomorrow, allowing me to review for one day before sending them to the director of LVSF. I am going on vacation in roughly 5 or 6 days, meaning this is when I would like to finish this more research and writing heavy portion of the project, allowing myself to work on much more enjoyable bit sweep code on my laptop during vacation.

## 07-18-26, Continued research plan and researched use cases

Today I worked on my research plan and began research on the use cases of a portable MCX algorithm. So far I have found spaceborne oceanic lidar as a promising lead, however I plan to finish research a bit tonight and tomorrow before implementing the findings into my research plan.

## 07-16-26, Started research plan, found Github Issue #41, further DSP research

Today I started on the research plan, compiling my specific sources and other references. I wasn't able to fully finish the research plan prototype, however I would like to take my time rather than rush through, as the research plan is very important to the project. 

Additionally and very excitingly, I found [Github issue #41](https://github.com/fangq/mcx/issues/41) while doing some final research for the research plan which perfectly outlines the issue of rounding errors slowly building up, and the current industry fixes for it, which notably does not include stochastic rounding. This is exciting as it is a core piece that validates the issue of swamping, and was found decently casually as the first result of Googling "MCX rounding accumulation". Further notes are in [notes.md](../citations/notes.md).

Finally, I looked over the research paper [DSP-Packing: Squeezing Low-precision Arithmetic into FPGA DSP Blocks](https://arxiv.org/abs/2203.11028) to learn more about DSP packing and found that I got some stuff wrong. For example, the DSP block actually features a pre adder on the 27 bit term along with a previous output term, meaning a single DSP cycle looks more like `(A + D) * B + C + prev` rather than just `A * B + C`. While I wasn't able to finish fully reviewing, a short skim let me note that their proposed format of "Overpacking" performed very well, and I'm interested in if it could be applicable to my application. Notes on paper are in [notes.md](../citations/notes.md).

## 07-15-26, LVSF forms, DSP optimizations, bit sweep reformatting

Today I looked over forms and began filling them out. I was a bit more busy with some other programming things, meaning I had only an hour or so to work today.

Additionally, I had some time to think about optimizations for the FPGA which I would like to get into writing. First, I was doing some research on DSPs, and found that DSPs have a similar form to a GPU `fma` function, meaning they multiply two numbers and add a third no matter what, with the two numbers being multiplied being 26 bits * 17 along with a free sign bit.

(from this doc https://docs.amd.com/v/u/en-US/ug579-ultrascale-dsp, page 9/77 from memory. I need to get this in TODO citations)

This works nicely with the -ln(eps) optimization of k ln(2) - ln(m). k ln(2) would be computed through shifts and adds, and ln(m) would be a piecewise function, meaning k ln(2) would be computed, the slope of -ln(m) would be computed, then a multiply-add would be used for the final result.

My main curiosity with this is for a SWAR like method, or "SIMD within a register" (SWAR seems to be an outdated term for CPUs, but it fits the idea), where you pack in multiple numbers to a multiply or other operation at the same expense as one large operation. Specifically, the unbalanced bit widths lend nicely to this, with the 26 * 17 being able to go down to a 13 * 17 and a 13 * 17. The main downside with this is that the 17 bit number has to be the same, however there are still a lot of uses for this. For example, in the scatter Henyey-Greenstein function, during the new direction multiplies, there are many multiplies with repeated terms, such as `dir.x * dir.z` and `dir.y * dir.z`, or `dir.x * cos_theta` and `dir.y * cos_theta`.

The biggest issue is that it would require the shared operand bits plus the 2 split bits to add to less than or equal to half of the output bits. In the case of the DSP, this has to be a combined 22 bits. If 17 bits was kept, this would leave only 5 bits for the two other terms each. This lends itself to 11 bits each, however is highly customizable. This is useful, as it means that a bunch of points at which I can trade each term for different bit sizes.

## 07-14-26, LVSF forms and serde parsing

Today and yesterday I did research on the requirements for the forms as I'm getting closer to experimentation. From what I found, if I want to do experimentation, I need to send my forms to the director of LVSF before doing anything that would generate data, so I plan to get that done tomorrow or the next few days.

Additionally, now that the JSON files are standardized, I wrote the Rust `serde` parsing code for the JSON, meaning I can start test runs for the reference, however I would like to finish my research plan and forms first.

## 07-12-26, Debugging and refactoring configs, research plan tomorrow

Today I looked over the debugging changes I made a few days ago and fixed some obvious oversights. More importantly, I refactored all the config system that I had previously left alone due to the fact that I will need to begin verifying if my reference implementation is working compared to a baseline. I found that I didn't actually need to ask AI to generate the JSON file converter outlined in [prompts.md](prompts.md), "Asked to build load_mcx_config in scripts/utils.py", and instead I just needed to use the `json2mcx` and `mcx2json` functions provided by pmcxcl.

This ended up taking a large amount of time due to the fact that all the JSON keys are completely different then the Python `cfg` keys. For example, the `vol` key in Python corresponds to the "Shapes" JSON key, with basically none lining up. Additionally, there is no actual dimensions field, meaning I had to implement my own. Since the amount of work in the scenes directory has grown, and since I plan to store my collected data there, I moved it up to the root directory.

Tomorrow, I plan to try to at least make a draft of my research plan. I'm still far away from collecting any data, but I want to get it done before I even run my reference. This should be fine as I only waited to write it until I fully understood the MCX algorithm, and now that I do after writing the reference, I should be ready to start.

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