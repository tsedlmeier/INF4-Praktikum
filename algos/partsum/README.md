# Praktikum 1: Maximale Abschnittssumme
Es soll die maximale Abschnittssumme für eine Abfolge von n ganzen Zahlen gefunden
werden. Die Zahlen liegen in einem Feld Z mit den Indizes 0 … n−1 im Speicher vor.
Hat man z.B. die Zahlenfolge
```
-59, 52, 46, 14, -50, 58, -87, -77, 34, 15
```
liefert folgende Teil-Zahlenfolge die maximale Abschnittssumme:
```
52 + 46 + 14 + -50 + 58 = 120
```
## Aufgabe 1
1) Implementieren Sie den betrachteten kubischen Algorithmus und ermitteln Sie für alle
vier Folgen die maximale Abschnittssumme (Wert, linke Grenze, rechte Grenze). 
<p align="center">
  <img src="./img/n3.png" width="50%" />
</p>
2) Messen Sie die Laufzeit Ihrer Lösung für die unterschiedlichen Längen der vorgegebenen Folgen.
Kann das kubische Wachstum bestätigt werden? 
3) Wodurch können Abweichungen entstehen?

## Lösung 1
<pre>
partsum
│   README.md
└───img
└───<b>n3</b>
└───nlogn
</pre>

## Aufgabe 2
1) Implementieren Sie den betrachteten "Teile-und-Herrsche"-Algorithmus (n*ld(n)).
Berechnen Sie ebenfalls die maximalen Abschnittssummen der gegebenen Folgen.
<p align="center">
  <img src="./img/nlogn.png" width="50%" />
  <img src="./img/zwi.png" width="40%" />
</p>

## Lösung 2
<pre>
partsum
│   README.md
└───img
└───n3
└───<b>nlogn</b>
</pre>

## Aufgabe 3
1) Implementieren Sie den betrachteten Algorithmus mit linearer Laufzeit ohne Verwendung eines Arrays/Vektors. 
Warum ist das aus Sicht der Effizienz eine gute Idee?
2) Berechnen Sie ebenfalls die maximalen Abschnittssummen der gegebenen Folgen und
vergleichen Sie die Ergebnisse.
