# Python Generator Implementations

The Pop Jump Push and Koda-Ruskey algorithms for generating all ideals of an arborescence are implemented along with a basic command line interface for exploring and benchmarking.

Use `--help` for argument details but the most important bits are
the output types

    0 performance timing
    1 algorithm arguments dump
    2 raw algorithm output
    3 ideals node output in native algorithm order
    4 ideals node output from native to sorted preorder (for comparing output)

and the sample data

    'set_Alpha', 'set_7Readme', 'set_Ruskey', 'set_13M',
    'set_3B', 'set_3W', 'set_3D',
    'set_7B', 'set_7W', 'set_7D',
    'set_15B', 'set_15W', 'set_15D',
    'set_31B', 'set_31W', 'set_31D',
    'set_53X',
    'set_63B'

The number represents the number of nodes and `B` is for a fully balanced tree, `W` for wide and `D` for deep. The others are for specific testing, like comparing Pop Jump Push output ordering to the Koda-Ruskey ordering.

`set_63B` takes a while on my machine when using python (pypy) so a shorter `set_53X` is available that executes in ~ 25 seconds when executed using pypy.

When using these algorithms in python I highly suggest using pypy (or another jit compiler). Compare the following `per ideal` timings, the first using `python` and the second using `pypy`.

`> python .\python\main.py -a 2 -o 0 -s set_53X -r 5`

```
Getting test set: set_53X
Generating 2,172,484,199 ideals from 53 nodes 5 times.

=== pop_jump_push ===

        Completed generating 10,862,421,000 ideals from 5 trees with 2,172,484,199 ideals.
        Avg Duration per tree 211.60530982017517
        Best Duration per tree 208.7090744972229
        97.40246208353443 ns avg per ideal
python->96.06931760115549 ns best per ideal

=== koda_ruskey ===

        Completed generating 10,862,421,000 ideals from 5 trees with 2,172,484,199 ideals.
        Avg Duration per tree 346.4034875869751
        Best Duration per tree 344.85082173347473
        159.45040601281497 ns avg per ideal
python->158.73570997303938 ns best per ideal
```

`> pypy .\python\main.py -a 2 -o 0 -s set_53X -r 5`

```text
Getting test set: set_53X
Generating 2,172,484,199 ideals from 53 nodes 5 times.

=== pop_jump_push ===

        Completed generating 10,862,421,000 ideals from 5 trees with 2,172,484,199 ideals.
        Avg Duration per tree 25.53632073402405
        Best Duration per tree 24.847885370254517
        11.754433355961108 ns avg per ideal
pypy -> 11.437544807779068 ns best per ideal

=== koda_ruskey ===

        Completed generating 10,862,421,000 ideals from 5 trees with 2,172,484,199 ideals.
        Avg Duration per tree 38.863981103897096
        Best Duration per tree 37.757262229919434
        17.889189307699584 ns avg per ideal
pypy -> 17.379763796348527 ns best per ideal
```

---
> Remember that this is _only_ benchmarking the generation algorithm and not an actual 'visit' on the generated ideals.

```python
    for _ideal in generator:
        i += 1
```
