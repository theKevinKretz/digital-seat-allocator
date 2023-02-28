---
title: Entwicklung eines digitalen Platzanweisers für Züge des Fernverkehrs
subtitle: "Erhöhung der Pünktlichkeit durch Reduzierung von Haltezeitüberschreitungen"
author: Kevin Kretz
date: \today
bibliography: [docs/main.bib]
csl: docs/ieee-with-url.csl
link-citations: true
documentclass: article
header-includes: |
    \usepackage[german]{babel}
    \usepackage[margin=4.0cm]{geometry}
numbersections: true
---

\maketitle

\newpage

\tableofcontents

\newpage

# Einleitung

Die Pünktlichkeit im Bahnverkehr ist ein zentrales Thema für viele Reisende und ein wichtiger Faktor für den Erfolg eines Bahnunternehmens. Leider hat die Deutsche Bahn Fernverkehr AG in den letzten Jahren immer wieder Probleme mit der Pünktlichkeit ihrer Züge gehabt. Im Jahr 2022 belief sich die Pünktlichkeit auf insgesamt rund 65 Prozent und hat sich damit gegenüber dem Vorjahr sogar verschlechtert. Dies bedeutet, dass mehr als ein Drittel aller Züge Verspätungen aufweisen und damit nicht den Erwartungen der Reisenden entsprechen. [@statista_DB_puenktlichkeit; @DB_puenktlichkeit]

Die Gründe für diese unzureichende Pünktlichkeit sind vielfältig und reichen von netzbedingten Ursachen über Störungen an Fahrzeugen und Lokomotiven bis hin zu Problemen mit der Leit- und Sicherungstechnik. Ein weiteres Problem ist die überalterte Infrastruktur sowie die intensive Bautätigkeit, die zu zusätzlichen Verzögerungen führen. [@DB_puenktlichkeit] Der Hauptgrund für Verspätungen sind jedoch Verzögerungen beim Ein- und Aussteigen, die zu Haltezeitüberschreitungen führen. [@presseinformationen_2023]

## Thema

Es ist bekannt, dass die Züge des größten deutschen Eisenbahnunternehmen, der Deutschen Bahn, bedeutend unpünktlicher sind als Züge in den meisten anderen Ländern. Um dieses Problem anzugehen, ist die Reduzierung der Verweildauer eines Zugs im Bahnhof förderlich. Das Thema dieser Arbeit ist die Entwicklung eines digitalen Platzanweisers, der genau dieses Ziel verfolgt. Ein solches System kann darüber hinaus den Fahrgastkomfort erhöhen, indem es eine lästige Suche nach einem geeigneten und freien Sitzplatz erspart.

## Relevanz

Dass die Deutsche Bahn AG grundsätzlich Interesse an einem solchen System hat, zeigen die Bemühungen, ein ähnliches System für den Regionalverkehr einzuführen. [@presseinformationen_2023]
Das am 8. Januar 2023 in einer Pressemitteilung angekündigte System soll durch ein dreistufiges Ampelsystem anzeigen, wie hoch die Auslastung in bestimmten Zugteilen ist. Die Einführung dieses Systems ist jedoch aufgrund der Notwendigkeit von Sensoren nicht nur teuer, sondern auch schlecht skalierbar, weshalb es planmäßig bis Ende 2024 in gerade einmal einem Viertel der Regionalverkehrszüge zum Einsatz kommen wird. [@presseinformationen_2023]

# Kapitel I: Konzeption

Das Grundkonzept für den digitalen Platzanweiser basiert darauf, Passagiere gezielt zu freien Sitzplätzen zu lotsen. Hierzu werden verschiedene Daten genutzt anhand derer zunächst freie Sitzplätze ermittelt und im Anschluss geeignete Plätze nach den individuellen Wünschen des Fahrgasts vorgeschlagen werden.

## Anforderungen

Bei der Entwicklung eines Konzepts für einen digitalen Platzanweiser gilt es, eine Reihe von Anforderungen zu erfüllen. Diese werden im folgenden erläutert.

### Genauigkeit

Zuverlässig akkurate Ergebnisse zu generieren ist die Grundlage der Arbeit.
Der vorgeschlagene Sitzplatz soll möglichst über die gesamte Fahrtdauer hinweg frei sein.

<!--#### Unterstützung unterschiedlicher ICE-Typen

Das Programm unterstützt verschiedene Arten von ICE-Zügen (z.B. ICE 1 bis 4) und kann die Sitzplatzverfügbarkeit für jeden Typ entsprechend erfassen und anzeigen.-->

#### Variable Start- und Endpunkte von Fahrten und Reservierungen

Das Programm ist in der Lage, variable Start- und Endpunkte von Fahrten und Reservierungen zu berücksichtigen und die Sitzplatzverfügbarkeit entsprechend anzupassen.

#### Berücksichtigung unterschiedlicher Anforderungen der Gäste

Das Programm bietet die Möglichkeit, unterschiedliche Anforderungen der Gäste zu berücksichtigen, z.B. die Wahl von bestimmten Sitzplatzkategorien oder die Berücksichtigung von Behinderungen.

### Skalierbarkeit

Wenn das softwarebasierte Projekt steht, kann es mit wenig Aufwand einer Große Zahl von Zügen und Fahrgästen zur Verfügung gestellt werden.
Dies wird ermöglicht, indem ein kosteneffizienter Ansatz ohne benötigtte Sensoren verfolgt wird.

### Multi-Tenancy

Mehrere Nutzer sollen das System gleichzeitig verwenden können.
Das Programm soll nicht mehrere zum gleichen Sitzplatz lotsen.

### Erweiterbarkeit

Das System kann einfach um zusätzliche Datenquellen erweitert werden.

### Datenschutz

Das System ist datenschutzfreundlich gestaltet.
Es erfüllt die strengen deutschen und europäischen Standards im Datenschutz.
So ist für die Verwendung des Programms keine Angabe personenbezogener Daten notwendig. - Übermittelt werden lediglich solche Daten, die zu einer Verbesserung der Ergebnisse beitragen. Welche Daten über welche Schnittstelle übermittelt werden, ist der entsprechenden Schnittstellenbeschreibung zu entnehmen (siehe unten).
Solche Daten stellen ein Berechtigtes Interesse gem. Art. 6 Abs. 1 Bst. f DSGVO dar. <!--SRC-->

## Ansatz

Diese Grafik veranschaulicht die Gesamtstruktur des Programms.

![Programmstruktur - Übersicht](docs/paper/assets/programmstruktur.tiff)

## Datenquellen

Das System ist in der Lage, Daten aus unterschiedlichen Datenquellen miteinzubeziehen, um die Sitzplatzverfügbarkeit im Zug zu ermitteln.
Ein späteres Hinzufügen weiterer Datenquellen ist möglich.

### Verwendete Daten

Verwendet werden hierzu Daten zu Reservierungen, zur Nutzung des Komfort Check-Ins, den eingeloggten WLAN-Geräten in bestimmten Wagons, sowie zu den Positionen kontrollierter Fahrgäste.
Im Folgenden werden die verwendeten Eingangsdaten beschrieben und erklärt, inwiewert diese hilfreich sind.

### Alternativen und Erweiterungsmöglichkeiten

Im Folgenden wird auf alternative Datenquellen eingegangen und erläutert, weshalb diese zunächst nicht verwendet werden.
Die Alternativen stellen jedoch auch potenzielle Erweiterungmöglichkeiten für das System dar, welche sich weitestgehend einfach implementieren ließen.

#### Sensoren

Eine mögliche Alternative Datenquelle sind Sensoren

Es wurden bereits Sensoren entwickelt, die unter dem Sitz platziert werden und Plätze auf deren Belegtheit hin überprüfen können. [@novak2012freeseat]

Der Einsatz von Sensoren ist jedoch mit Kosten für Anschaffung, Installation und Wartung verbunden. <!--SRC-->
Stattdessen auf intelligente und flexible Softwarelösungen zu setzen ist erfolgversprechender. [@porter2015smart]

#### Kameras

Die Erfassung belegter Sitzplätze wäre zwar technisch möglich, jedoch birgt ein solches System ein Risiko für Missbrauch und Hackerangriffe.

Auch schränkt die dauerhafte Verwendung der aufgenommenen Videodaten die Freiheiten der Fahrgäste ein.
Berechtigte Interessen der DB sind hierbei bisher lediglich *"die Aufklärung und Verhinderung von Straftaten sowie der Schutz von Leben, Gesundheit und Freiheit von Beschäftigten und Kunden, sowie die Beweissicherung im Ereignisfall"*. [Deutsche Bahn] <!--SRChttps://www.dbregio.de/home/datenschutz-video--> 

Diese Interessen aus Komfortgründen zu erweitern wäre nicht richtig.

## Algorithmen

### Datengenerator

Zunächst wird ein Programm entwickelt, welches realistische Daten generiert, die es dem System zur Verfügung stellt.

Durch Anpassung der Parameter können verschiedene Verhaltensweisen von Fahrgästen simuliert werden.
Außerdem ist ein Sammeln echter Daten zunächst nicht nötig.

Spätere Studien mit echten Daten werden, sofern möglich, durchgeführt.

#### Simulation des Fahrgastverhaltens

Die Auswahl der Sitzplätze durch die Fahrgäste wird simuliert. Das Verhalten von Fahrgästen beim Einstieg in Züge zeigt, welche Eigenschaften ihnen besonders wichtig sind.
Leere Sitzgruppen werden anderen stark vorgezogen. Die Sitzgruppe, in der die kleinste Anzahl von Menschen sitzt, wird den anderen vorgezogen. Die Sitze in Fahrtrichtung sind beliebter als die Sitze entgegen der Fahrtrichtung. Sitze an Fenstern sind beliebter als Sitze an den Gängen. Der Sitzplatz, der diagonal zum besetzten Sitzplatz liegt wird gegenüber der anderen bevorzugt. [@schottl2017investigating]

Diese Regeln finden sich in der Programmstruktur des Datengenerators als "*Verhalten*" wieder.

#### Berechnung der Eingangsdaten

Die finalen Daten, mit denen im nächsten Schritt der Wahrscheinlichkeitsgenerator arbeiten kann, werden wie folgt generiert:

* Zunächst wird ein zufälliger Wert $0,2 < k_{d/p} < 0,9$ generiert, der die durchschnittliche Zahl der Geräte $d$ pro Fahrgast $p$ angibt. Dieser ist zufällig, da er stark von den Gebieten abhängt, durch welche ein Zug fährt und ansonsten unabhängig von den anderen generierten Daten ist. Der Wert dient weiterhin lediglich der Ermittlung der relativen Anzahl von Fahrgästen pro Wagon (siehe unten), weshalb seine Größe kaum eine Rolle spielt.
* Gleiches wird für den Anteil der Fahrgäste, die den Komfort Check-in bzw. eine Reservierung nutzen, getan. Hierbei ist der Bereich, in dem sich der Wert befindet, kleiner.
* Der Kontrolleur läuft in der Simulation durch den Zug und erfasst die Daten der Fahrgäste, die er kontrolliert. Sie stehen ab dem Zeitpunkt der Kontrolle dem System zur Verfügung.

Hieraus lassen sich nun mit einfacher Mathematik die Daten berechnen, die dem Programm zur Verfügung gestellt werden.

#### Programmstruktur des Datengenerators

Die folgende Grafik veranschaulicht die Struktur des Datengenerators.

![Programmstruktur - Datengenerator](docs/paper/assets/datengenerator.tiff)

### Wahrscheinlichkeiten für Sitze ermitteln

Es wird ein Programm entwickelt, welches aus bestimmten Daten die Auslastung des Zuges in bestimmten Bereichen ermittelt.
Ziel ist es hierbei, möglichst genaue Angaben zur Sitzplatzbelegung zu machen.
Zunächst werden die Zusammenhänge von Eingangsdaten zur Belegung der Sitzplätze mathematisch beschrieben.

#### Definitonen

Die (absolute) Auslastung beschreibt den Anteil der belegten Plätze.

Die relative Auslastung beschreibt den Anteil der belegten Plätze in einem Bereich im Vergleich zu einem anderen Bereich.

#### Zug

Ein Zug $train$ hat $s_{train}$ Sitzplätze und $c_{train}$ Fahrgäste.

##### Sitzplätze

Die Zahl der Sitzplätze eines Zugs ist abhängig von dessen Typ.
Dieser kann anhand der Zugnummer ermittelt werden.

##### Auslastung

Die Auslastung $a_{train}$ des Zuges $train$ ist:

$a_{train} = \frac{c_{train}}{s_{train}}$

##### Wahrscheinlichkeit für besetzten Platz

Die Wahrscheinlichkeit $p_{train, random}$, dass ein zufälliger Sitzplatz besetzt ist liegt (unter der Annahme, dass jede Person im Waggon einen Sitzplatz einnimmt) bei:

$p_{train, random} = \frac{c_{train}}{s_{train}}$


#### Waggons

Die Wahrscheinlichkeit $p_{coach, random}$, dass ein zufälliger Sitzplatz $random$ in einem Waggon $coach$ mit $s_{coach}$ Sitzplätzen und $c_{coach}$ Personen belegt ist, ergibt sich (unter der Annahme, dass jede Person im Waggon einen Sitzplatz einnimmt) aus:

$p_{coach, random} = \frac{c_{coach}}{s_{coach}}$

#### WLAN-Geräte

Es ist möglich, die Zahl der WLAN-Geräte $d_{coach}$ in einem Waggon zu bestimmen. <!--SRC-->
Dies gibt Auskunft über die Verteilung von Personen innerhalb eines Zugs.

Die Zahl $k_{d/p}$ der eingeloggten WLAN-Geräte $d$ pro Person $p$ lässt sich von den Gebieten beeinflussen, durch welche der Zug fährt. Funktioniert das WLAN nicht, da der Zug keine zuverlässige Netzwerkverbindung aufbauen kann, so nutzen werniger Fahrgäste den Service. <!--SRC-->
Aus diesem Grund lässt sich aus der Gerätezahl $d$ allein keine absolute Zahl $c_{coach}$ der Fahrgäste in einem bestimmten Waggon $coach$ bestimmen. Die Daten helfen jedoch maßgeblich dabei, die relative Auslastung $a_{rel, coach}$ eines Waggons $coach$ im Zug $train$ mit insgesamt $d_{train}$ WLAN-Geräten zu bestimmen.
Dennoch lassen sich Rückschlüsse auf die relative Auslastung $a_{rel, coach}$ eines Waggons $coach$ ziehen.

Hierbei ist es wichtig, zu beachten, dass nicht jeder Waggon gleich viele Sitzplätze hat.

##### Relative Zahl von Sitzplätzen $s_{rel, coach}$

Die relative Anzahl von Sitzplätzen $s_{rel, coach}$ in einem Waggon $coach$ ist:

$s_{rel, coach} = \frac{s_{coach}}{s_{train}}$


##### Relative Zahl von Fahrgästen $c_{rel, coach}$

Die relative Anzahl von Fahrgästen $c_{rel, coach}$ in einem Waggon $coach$ ist:

$c_{rel, coach} = \frac{d_{coach}}{d_{train}}$


##### Relative Auslastung $a_{rel, coach}$

Die relative Auslastung $a_{rel, coach}$ ergibt sich nun aus:

$a_{rel, coach} = \frac{c_{rel, coach}}{s_{rel, coach}}$

Alternative Form:

$a_{rel, coach} = \frac{d_{coach} \div d_{train}}{s_{coach} \div s_{train}} = \frac{d_{coach} \times s_{train}}{s_{coach} \times d_{train}}$


##### Absolute Zahlen

Kennt man die Fahrgastzahl $c_{train}$ im Zug, so lassen sich die absoluten Zahlen berechnen.

Die Zahl $k_{d/p}$ ist innerhalb eines Zuges gleich, was die Berechnung von $c_{coach}$ mithilfe der Zahl $c_{train}$ zulässt. <!--SRC-->

##### Absolute Zahl von Sitzplätzen $s_{coach}$ (Der Vollständigkeit halber)

$s_{coach} = s_{rel, coach} \times s_{train}$


##### Absolute Zahl von Fahrgästen $c_{coach}$

$c_{coach} = c_{rel, coach} \times c_{train}$


##### Absolute Auslastung $a_{coach}$

Kennt man die Fahrgastzahl $c_{train}$ im Zug, so lassen sich die absoluten Zahlen berechnen.
Aus der ermittelten relativen Auslastung eines Waggons $a_{rel, coach}$ und der absoluten Auslastung des Zuges $a_{train}$ lässt sich die absolute Auslastung $a_{coach}$ im Waggon ermitteln.

$a_{coach} = a_{rel, coach} \times a_{train} = \frac{d_{coach} \times s_{train}}{s_{coach} \times d_{train}} \times a_{train}$


##### Zahl der WLAN-Geräte pro Fahrgast $k_{d/p}$

Wenn man annimmt, dass ein Fahrgast durchschnittlich mit $k_{d/p}$ Geräten im WLAN eines Waggons ${coach}$ angemeldet ist, so ergibt sich für einen Waggon ${coach}$ eine Personenzahl $c_{coach}$ von:

$c_{train} = k_{d/p} \times d_{coach}$

somit beläuft sich die Wahrscheinlichkeit besetzt zu sein $p_{random}$ für einen zufälligen Sitzplatz $random$ auf:

$p_{random} = \frac{k_{d/p} \times d}{s_{coach}}$

#### Reservierungen

Eine zuverlässige Auskunft über die Auslastung im Zug bieten die Reservierungsdaten anderer Gäste. <!--SRC-->
Die Reservierung von Sitzplätzen hat Auswirkungen auf die $k_{res}$ belegt.

Die Wahrscheinlichkeit besetzt zu sein $p_{reserved}$ ist:

$p_{reserved} = p_{random} + (1-p_{random}) * k_{res}$

#### Komfort Check-in

Bei Nutzung des Komfort Check-in bestätigt der Fahrgast, dass er sich auf einem bestimmten Sitzplatz befindet. Dieser kann im Voraus reserviert worden sein, muss aber nicht. <!--SRC-->
Entsprechende Sitzplätze sind zu $k_{KCI}$ belegt.

Die Wahrscheinlichkeit besetzt zu sein $p_{KCI}$ ist:

$p_{KCI} = p_{random} + (1-p_{random}) * k_{KCI}$

#### Kontrollierte Plätze


Kontrollierte Plätze sind zu einer Wahrscheinlichkeit von $k_{con}$ tatsächlich belegt.
    Fehler des Kontrolleurs möglich (eingerechnet)
    Umsetzen möglich (muss extra eingerechnet werden)


$p_r = x$

#### $k$-Werte

Die Zahl der Geräte pro Person muss nicht genau bestimmt werden (siehe Ausführungen oben).

$k_{d/p} = X$

Ein Großteil der Reservierung wird wahrgenommen. Nach einer eigenen Umfrage liegt dieser Wert bei 80%.
Eventuell wird der Wert angepasst, wenn aussagekräftigere Daten vorliegen.
<!-- In einem vollen Zug machen mehr Fahrgäste von einer Reservierung Gebrauch als in einem leeren. SRC -->
$k_{res} = 80\%$

Es ist davon auszugehen, dass Fahrgäste größtenteils gewissenhafte Annahmen machen. <!-- SRC -->
$k_{KCI} = 95\%$

Die Daten von Kontrolleuren sind sehr zuverlässig, da sie sehen, wo sich Fahrgäste befinden. Da menschliche Fehler jedoch nicht auszuschließen sind, gilt:
$k_{con} = 99\%$

### Optimalen Sitzplatz / Sitzgruppe wählen

Die Daten zur Auslastung werden an den eigentlichen Platzanweiser (PA) übermittelt, der den Fahrgästen die Sitzplätze zuweist.
Das Programm funktioniert durch ein Zusammenspiel mehrerer Teile.


#### Folgerungen aus dem Fahrgastverhalten

Aus dem Fahrgastverhalten (siehe weiter oben) werden Regeln abgeleitet, nach denen die Qualität eines Platzes bewertet wird.

Je weniger Menschen in einer Sitzgruppe sitzen, desto beliebter ist diese.
Je mehr beliebte Sitzgruppen in einem Zugteil, desto beliebter. [@zhang2008modeling]

Daraus folgt: Nicht die totale Anzahl von Menschen in einem Zugteil ist relevant, sondern auch deren Verteilung.

<!--
Wenn der Passagier sitzen will:
Zugteil wählen
Sitzgruppe wählen
Sitzplatz wählen -->

Aus den Zahlen der Studie lassen sich außerdem Standardregeln zum Ranking der Qualität von Sitzplätzen erstellen. [@zhang2008modeling]

#### Benutzerdefinierte Regeln zum Ranking

Der Nutzer soll auch selbst Kriterien festlegen können, die ihm bei der Wahl eines Sitzplatzes wichtig sind.
Vielen Menschen sind hierbei die folgenden Eigenschaften eines Platzes Wichtig: <!--SRC-->

* Platz am Fenster / Platz am Gang (binär)
* Entfernung zum Speisewagen (float)
* Entfernung von Waggonenden (float)
* 1. oder 2. Klasse / Kinderabteil / etc. (je binär)

*Erläuterungen:* Plätze an den Enden von Wagons sind näher an den Toiletten und (Not-)Ausgängen. Sie sind jedoch weniger geeignet für Menschen, denen schnell übel wird, da dort die Auslenkung des Zugs in Kurven höher ist als in der Mitte des Waggons.

Die mit *float* gekennzeichneten Werte sind Werte zwischen Null und Eins. Ist ein Platz beispielsweise in der Mitte eines Waggons, so erhält er die für die Eigenschaft *Entfernung von Waggonenden* den Wert $1$. Befindet er sich am Ende eines Waggons, so ist der Wert gleich $0$.

Die Eignungen der einzelnen Werte lässt sich wie folgt berechnen (Beispiel für Entfernung vom Speisewagen):

$v_{distance to dining car} = \frac{1}{|optimum - tatsächlicher Wert|}$

Der Wert liegt zwischen Null und Eins.

Der Nutzer hat die Möglichkeit, die einzelnen Kiterien verschieden zu gewichten.
Die Formel zur Berechnung der Eignung $v_{seat}$ eines Platzes ${seat}$ lautet:

$v_{seat} = \frac{w_1 \times v_{window/aisle} + w_2 \times v_{distance to dining car} + ...}{sum of weights}$

Hierbei stellen $w_1$, $w_2$, etc. die Gewichtungen für die jeweiligen Eigenschaften dar und $v_{window/aisle}$, $v_{distance_to_dining_car}$, etc. sinddie Werte für die Eigenschaften des Platzes.

Der Nutzer kann die Gewichtungen anpassen, um die Eigenschaften zu priorisieren, die ihm am wichtigsten sind.
Je höher der Faktor einer Eigenschaft ist, desto stärker beeinflusst dieser Eigenschaft die Eignung des Platzes.

Zuletzt kann der Nutzer einen Risikofaktor $r$ festlegen.
Dieser gibt die Gewichtung von der Wahrscheinlichkeit frei zu sein $p_{seat}$ und der Eignung $v_{seat}$ an.
Die Formel für den Gesamtscore $score_{seat}$ eines Sitzplatzes lautet:

$score_{seat} = p_{seat} + r * v_{seat}$

### Auswahl und Ranking der Optionen

Zur Auswahl der möglichen Sitzplätze wird ein Algorithmus verwendet, der eine Kombination aus Gesamtscore $score_{seat}$ und gewünschter Anordnung der Sitzplätze berücksichtigt.
Es werden mehrere Algorithmen implementiert und Simulationen durchgeführt, um den besten zu ermitteln.

#### Multi-Objective Optimization Algorithm
Eine Möglichkeit ist ein sogenannter *Multi-Objective Optimization Algorithm*, wie der *Non-dominated Sorting Genetic Algorithm (NSGA-II)*. <!--SRC-->
Dieser Algorithmus kann mehrere Ziele gleichzeitig berücksichtigen und ein Ranking der möglichen Plätze aufgrund der Prioritäten des Nutzers erstellen.

#### Alternativen
Auch die Verwendungen eines *Decision Tree*- oder eines *Random Forest*-Algorithmus stellen Optionen dar. <!--SRC-->
Darüber hinaus dienen *Genetische Algorithmen* der Berechnung.

### Multi-Tendency

Damit eine hohe Zahl von Nutzern das System nutzen kann, ist es nötig, dass die einzelnen Instanzen des Programms miteinander im Austausch stehen und so verhindern, dass mehrere Personen zum gleichen Sitzplatz gelotst werden.

## Simulation

### Ziele und Einschränkungen

Simulationen spiegeln nicht zwangsläufig die Realität wider. Das kann dazu führen, dass sich die gewonnenen Ergebnisse nicht vollumfänglich in der Realität beobachten lassen.

Der Vorteil von Computersimulationen ist jedoch, dass sich durch sie schnell und ohne großen Aufwand die Qualität des Systems in verschiedensten Bedingungen bewerten lässt.
Die Simulation hilft dabei, Vorgänge im System zu verstehen und sie zu optimieren.

### Herangehensweise

<!-- ADD CONTENT -->

## Schnittstellen

Damit es möglichst einfach ist, von dem Programm zu profitieren, müssen Syssteme von Apps und Bahnunternehmen mit dem Programm kommunizieren können.
Um diese Kommunikation zwischen Computern zu ermöglichen, sind Schnittstellen erforderlich.
Eine Schnittstelle definiert Regeln und Protokolle für die Übertragung von Daten und die Interaktion der Systeme.
Sie stellt eine gemeinsame Sprache für die Übertragung von Daten bereit.
Die implementierten Schnittstellen nutzen das JSON-Format zum Austausch von Daten.
Das Format eigent sich für diesen Zweck, da es sprachenunabhängig und für den Menschen gut lesbar ist, was es einfach macht, Schnittstellen zu verknüpfen. <!--SRC-->

Im folgenden werden die Schnittstellen des Programms, sowie deren Zweck und der Umfang der auszutauschenden Daten erläutert.

### Zur Datenerhebung

Die Schnittstellen zur Datenerhebung muss den entsprechenden APIs des Bahnunternehmens angepasst werden. Die Deutsche Bahn AG stellt unter anderem im *API Marketplace* Lösungen zur Verfügung.
Es sind mehrere verschiedene Schnittstellen nötig, da keine einzelne API alle Daten zur Verfügung stellt.

#### Ausgetauschte Daten
* ICE-Typ
* ICE-Nummer
* Informationen zum Streckenverlauf
* Daten zu Reservierungen
  * Einstiegsbahnhof
  * Ausstiegsbahnhof
* Daten zum Komfort Check-in
  * Position
  * Ausstiegsbahnhof
* Daten zu WLAN-Geräte in Waggons
  * Waggon
* Daten zu kontrollierten Gästen
  * Ausstiegsbahnhof

### Für andere Instanzen

Zur Kommunikation mit anderen Instanzen des Programms ist ebenfalls eine Schnittstelle nötig.

#### Ausgetauschte Daten
* ICE-Nummer
* Einstiegsbahnhof
* Ausstiegsbahnhof
* Empfohlene Sitzplätze für Nutzer
* ggf. Anforderungen des Nutzers

### Für Apps

#### Eingehende Daten
* ICE-Nummer
* Einstiegsbahnhof
* Ausstiegsbahnhof
* Anforderungen des Nutzers
* ggf. Position des Nutzers

#### Ausgehende Daten
* Empfohlene Sitzplätze für Nutzer

### Für Züge und Bahnhöfe

#### Eingehend
* ICE-Nummer

#### Ausgehend
* Auslastungen einzelner Waggons

# Quellen