# Praktikum 2: Merge-Sort
Ein bisher nicht betrachtetes Sortierverfahren ist der sog. Merge-Sort. Es handelt sich dabei
wieder um einen "Teile-und-Herrsche"-Algorithmus. Die Grundidee ist dabei folgende:
1) Um ein Array zu sortieren betrachtet man zunächst die
Länge des Arrays. Hat es die Länge 1, ist es sortiert und
wir sind fertig.
2) Andernfalls teilen wir das Array in zwei möglichst gleich
große Teilarrays (links und rechts) und rufen das
Sortierverfahren für jedes erneut rekursiv auf.
Anschließend müssen die beiden sortierten Teilarrays
zusammengeführt werden ("Merge")

```c
-------------------
MERGESORT(Z,l,r)
-------------------
if l = r then
    return (Z,n)
end if

m = (l+r)/2
sorted_left = MERGESORT(Z,l,m)
sorted_right = MERGESORT(Z,m+1,r)

nl = length(sorted_left)
nr = length(sorted_right)
i=0, j=0, n=0
while (i<nl && j<nr) do
    if sorted_left[i] < sorted_right[j] then
        sorted[n] = sorted_left[i]
        i++
        n++
    else
        sorted[n] = sorted_right[j]
        j++
        n++
    end if
end while
while

while (i<nl)
    sorted[n] = sorted_right[i]
    i++
    n++
end while

while (j<nr)
    sorted[n] = sorted_right[j]
    j++
    n++
end while

return (sorted)
```

```rust
fn mergesort(z: &Vec<i8>, l: usize, r: usize) -> Vec<i8>
{
    if l == r {
        return vec![ z[l] ];
    }

    let m:usize = (l+r)/2;
    let sorted_left = mergesort(&z, l, m);
    let sorted_right = mergesort(&z, m+1, r);

    let mut sorted: Vec<i8> = Vec::new();
    let mut i=0;
    let mut j=0;
    while  i<sorted_left.len() && j<sorted_right.len() {
        if sorted_left[i] < sorted_right[j]{
            sorted.push(sorted_left[i]);
            i+=1;
        }
        else {
            sorted.push(sorted_right[j]);
            j+=1;
        }
    }
    // Push back the rest
    while i<sorted_left.len() {
        sorted.push(sorted_left[i]);
        i+=1;
    }
    while j<sorted_right.len() {
        sorted.push(sorted_right[j]);
        j+=1;
    }

    return sorted;
}
```
