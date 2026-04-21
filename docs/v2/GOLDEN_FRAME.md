# YAM v2 Golden Frame

This file captures the current deterministic baseline output of the v2 scaffold.

The baseline uses a fixed clock value so the output stays comparable.

## Generation Command

```bash
cd ../../v2
go run ./cmd/yamv2 --once --width 40 --height 20 --clock 12:34 --day Tuesday
```

## Frame

```text
                                        
                                        
                                        
           ______    _____ __ __        
          <  /__ \ _|__  // // /        
          / /__/ /(_)/_ </ // /_        
         / // __/_ ___/ /__  __/        
        /_//____(_)____/  /_/           
                                        
                                        
                                        
                Tuesday                 
   ____ ______  _____ __ __  ___________
  / __ <  /__ \|__  // // / / ____/ ___/
 / / / / /__/ / /_ </ // /_/___ \/ __ \ 
/ /_/ / // __/___/ /__  __/___/ / /_/ / 
\____/_//____/____/  /_/ /_____/\____/ /
                                        
                                        
                                        
```

## Notes

- this is the current FIGlet baseline for the v2 scaffold
- update this file only when the intended baseline output changes
- keep it aligned with the Go one-shot FIGlet renderer and the Python check harness
