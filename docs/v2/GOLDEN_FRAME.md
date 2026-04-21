# YAM v2 Golden Frame

This file captures the current deterministic baseline output of the v2 scaffold.

The baseline uses a fixed clock value so the output stays comparable.

## Generation Command

```bash
cd ../../v2
go run ./cmd/yamv2 --once --width 40 --height 20 --clock 12:34 --day "wtorek, 21 kwietnia"
```

## Frame

```text
                                        
                                        
                                        
                                        
                                        
                ||   ''|,    ,'''|,    /
               '||  '  || ||     ||  // 
                ||    .|'     '''|| //..
                ||   //   ||     ||     
               .||. ((...    '...|'     
                                        
                    wtorek, 21 kwietnia 
                                        
                                        
                                        
                                        
                                        
                                        
          q quit •   space pause        
                                        
```

## Notes

- this is the current FIGlet baseline for the v2 scaffold
- update this file only when the intended baseline output changes
- keep it aligned with the Go one-shot FIGlet renderer and the Python check harness
