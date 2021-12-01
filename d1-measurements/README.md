Count the number of times a depth measurement increases from the previous
measurement. (There is no measurement before the first measurement.) In the
example above, the changes are as follows:

```
199 (N/A - no previous measurement)
200 (increased)
208 (increased)
210 (increased)
200 (decreased)
207 (increased)
240 (increased)
269 (increased)
260 (decreased)
263 (increased)
```

In this example, there are 7 measurements that are larger than the previous
measurement.

Using the [measurements.txt](measurements.txt) file, how many measurements are
larger than the previous measurement.
