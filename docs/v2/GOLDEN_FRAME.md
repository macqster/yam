# YAM v2 Golden Frame

This file captures the current deterministic baseline output of the v2 scaffold.

The baseline uses a fixed clock value so the output stays comparable.

## Generation Command

```bash
python3 - <<'PY'
from v2.app import build_demo_model, build_demo_ecosystem
from v2.runtime.system import render_frame_with_clock
m = build_demo_model()
e = build_demo_ecosystem()
print(render_frame_with_clock(m, e, clock_text="12:34"))
PY
```

## Frame

```text
      ·······················   12:34   
     ···························        
    ·····························       
   ·······························      
   ·················█············       
    ············+···············        
     ··························         
       ·······················          
    ·······························     
  ·································     
 ··································     
 ··································     
```

## Notes

- this is the first visual contract for the v2 scaffold
- update this file only when the intended baseline output changes
- keep it aligned with the current deterministic demo state
