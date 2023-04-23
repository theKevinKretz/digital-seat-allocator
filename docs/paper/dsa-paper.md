---
title: Entwicklung eines digitalen Platzanweisers für Züge des Fernverkehrs
subtitle: "Erhöhung der Pünktlichkeit durch Reduzierung von Haltezeitüberschreitungen"
author: Kevin Kretz
date: \today
# abstract: |
#     SRC Abstract
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

* **Sitzplätze**  
  Die Zahl der Sitzplätze eines Zugs ist abhängig von dessen Typ.
  Dieser kann anhand der Zugnummer ermittelt werden.

* **Auslastung**  
  Die Auslastung $a_{train}$ des Zuges $train$ ist:  
  $a_{train} = \frac{c_{train}}{s_{train}}$

* **Wahrscheinlichkeit für besetzten Platz**  
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

* **Relative Zahl von Sitzplätzen $s_{rel, coach}$**  
  Die relative Anzahl von Sitzplätzen $s_{rel, coach}$ in einem Waggon $coach$ ist:  
  $s_{rel, coach} = \frac{s_{coach}}{s_{train}}$

* **Relative Zahl von Fahrgästen $c_{rel, coach}$**  
  Die relative Anzahl von Fahrgästen $c_{rel, coach}$ in einem Waggon $coach$ ist:  
  $c_{rel, coach} = \frac{d_{coach}}{d_{train}}$

* **Relative Auslastung $a_{rel, coach}$**  
  Die relative Auslastung $a_{rel, coach}$ ergibt sich nun aus:  
  $a_{rel, coach} = \frac{c_{rel, coach}}{s_{rel, coach}}$  
  Alternative Form:  
  $a_{rel, coach} = \frac{d_{coach} \div d_{train}}{s_{coach} \div s_{train}} = \frac{d_{coach} \times s_{train}}{s_{coach} \times d_{train}}$

* **Absolute Zahlen**  
  Kennt man die Fahrgastzahl $c_{train}$ im Zug, so lassen sich die absoluten Zahlen berechnen.  
  Die Zahl $k_{d/p}$ ist innerhalb eines Zuges gleich, was die Berechnung von $c_{coach}$ mithilfe der Zahl $c_{train}$ zulässt. <!--SRC-->

* **Absolute Zahl von Sitzplätzen $s_{coach}$ (Der Vollständigkeit halber)**  
  $s_{coach} = s_{rel, coach} \times s_{train}$

* **Absolute Zahl von Fahrgästen $c_{coach}$**  
  $c_{coach} = c_{rel, coach} \times c_{train}$

* **Absolute Auslastung $a_{coach}$**  
  Kennt man die Fahrgastzahl $c_{train}$ im Zug, so lassen sich die absoluten Zahlen berechnen.  
  Aus der ermittelten relativen Auslastung eines Waggons $a_{rel, coach}$ und der absoluten Auslastung des Zuges $a_{train}$ lässt sich die absolute Auslastung $a_{coach}$ im Waggon ermitteln.  
  $a_{coach} = a_{rel, coach} \times a_{train} = \frac{d_{coach} \times s_{train}}{s_{coach} \times d_{train}} \times a_{train}$

* **Zahl der WLAN-Geräte pro Fahrgast $k_{d/p}$**  
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

#### Kontrollierte Plätze <!-- ERGÄNZEN -->

Kontrollierte Plätze sind zu einer Wahrscheinlichkeit von $k_{con}$ tatsächlich belegt.
    <!-- Fehler des Kontrolleurs möglich (eingerechnet)
    Umsetzen möglich (muss extra eingerechnet werden) -->
$p_r = x$

#### $k$-Werte

* $k_{d/p}$
Die Zahl der Geräte pro Person muss nicht genau bestimmt werden (siehe Ausführungen oben).  
$k_{d/p} = X$

* $k_{res}$
Ein Großteil der Reservierung wird wahrgenommen. Nach einer eigenen Umfrage liegt dieser Wert bei 80%.
Eventuell wird der Wert angepasst, wenn aussagekräftigere Daten vorliegen.  
<!-- In einem vollen Zug machen mehr Fahrgäste von einer Reservierung Gebrauch als in einem leeren. SRC -->
$k_{res} = 80\%$

* $k_{KCI}$
Es ist davon auszugehen, dass Fahrgäste größtenteils gewissenhafte Annahmen machen.  <!-- SRC -->
$k_{KCI} = 95\%$

* $k_{con}$
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

$v_{distance to dining car} = \frac{1}{|optimum - actual value|}$

<!-- TODO use minimation / maximation instead -->

Der Wert liegt zwischen Null und Eins.

Der Nutzer hat die Möglichkeit, die einzelnen Kiterien verschieden zu gewichten.
Die Formel zur Berechnung der Eignung $v_{seat}$ eines Platzes ${seat}$ lautet:  
$v_{seat} = \frac{w_1 \times v_{window/aisle} + w_2 \times v_{distance to dining car} + ...}{sum of weights}$

Hierbei stellen $w_1$, $w_2$, etc. die Gewichtungen für die jeweiligen Eigenschaften dar und $v_{window/aisle}$, $v_{distance_to_dining_car}$, etc. sind die Werte für die Eigenschaften des Platzes.

Der Nutzer kann die Gewichtungen anpassen, um die Eigenschaften zu priorisieren, die ihm am wichtigsten sind.
Je höher der Faktor einer Eigenschaft ist, desto stärker beeinflusst dieser Eigenschaft die Eignung des Platzes.

Zuletzt kann der Nutzer ein Sicherheitsverhältnis $r$ festlegen.
Dieser gibt die Gewichtung von der Wahrscheinlichkeit frei zu sein $p_{seat}$ und der Eignung $v_{seat}$ an.
Je höher das Sicherheitsverhältnis, desto stärker wird $p_{seat}$ gegenüber $v_{seat}$ berücksichtigt.
Es gilt $0 \leq r \leq 1$.
Die Formel für den Gesamtscore $score_{seat}$ eines Sitzplatzes lautet:  
$score_{seat} = r * p_{seat} + (1-r) * v_{seat}$

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

### Datenerhebung

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

### Andere Instanzen

Zur Kommunikation mit anderen Instanzen des Programms ist ebenfalls eine Schnittstelle nötig.

#### Ausgetauschte Daten
* ICE-Nummer
* Einstiegsbahnhof
* Ausstiegsbahnhof
* Empfohlene Sitzplätze für Nutzer
* ggf. Anforderungen des Nutzers

### Apps

#### Eingehende Daten
* ICE-Nummer
* Einstiegsbahnhof
* Ausstiegsbahnhof
* Anforderungen des Nutzers
* ggf. Position des Nutzers

#### Ausgehende Daten
* Empfohlene Sitzplätze für Nutzer

### Züge und Bahnhöfe

#### Eingehend
* ICE-Nummer

#### Ausgehend
* Auslastungen einzelner Waggons

# Kapitel II: Struktur und Simulation

In diesem Kapitel wird die Struktur und Simulation des entwickelten Systems zur Vorhersage freier Sitzplätze in Zügen vorgestellt. Zunächst werden die grundlegenden Entscheidungsaspekte wie Rekonstruierbarkeit und Wahl der Programmiersprache erläutert. Anschließend werden die verschiedenen Module und Strukturen des Systems beschrieben, die für die Modellierung von Zügen, Passagieren und deren Interaktionen verantwortlich sind. Schließlich wird die Umsetzung einer Beispiel-Simulation vorgestellt, um die Funktionsweise des Systems zu verdeutlichen und dessen Anwendbarkeit zu demonstrieren.

Der angefertigte Code ist verfügbar unter: <https://github.com/theKevinKretz/digital-seat-allocator/tree/main/src>

## Grundlegende Entscheidungsaspekte

### Rekonstruierbarkeit als Grundlage für Falsifizierbarkeit
Um die wissenschaftliche Integrität des entwickelten Systems sicherzustellen, ist es wichtig, dass die Ergebnisse reproduzierbar und damit auch falsifizierbar sind. Dies wird durch folgende Maßnahmen erreicht:

  * **Offenlegung des Codes**: Der gesamte Quellcode des Systems wird öffentlich zugänglich gemacht, um eine unabhängige Überprüfung und Nachvollziehbarkeit der Ergebnisse zu ermöglichen.
  * **Dokumentation der verwendeten Bibliotheken**: Alle im System verwendeten externen Bibliotheken werden eindeutig aufgelistet, um die Nachvollziehbarkeit und Reproduzierbarkeit der Ergebnisse zu gewährleisten.
  * **Versionskontrolle mit Git**: Das System verwendet Git als Versionskontrollsystem, um die Historie der Codeänderungen nachvollziehbar zu machen und die Zusammenarbeit zwischen Entwicklern zu erleichtern.

### Wahl der Programmiersprache

Das System wird in der Programmiersprache Rust entwickelt. Rust ist eine leistungsstarke und sichere Programmiersprache, die sich besonders für die Entwicklung von skalierbaren und datenintensiven Systemen eignet. Die folgenden Vorteile von Rust haben zur Wahl dieser Programmiersprache geführt:

* **Performance**: Rust wurde für hohe Leistung entwickelt und bietet durch effiziente Speicherverwaltung und geringen Overhead eine verbesserte Performance. Dies ist besonders wichtig für datenintensive Systeme wie dieses.
* **Speichersicherheit**: Die Sprache verfügt über Mechanismen, um Speicherfehler wie Pufferüberläufe, Nullzeigerdereferenzierung und andere sicherheitskritische Fehler zu vermeiden. Spätenstens beim Bedienen mehrerer Nutzer ist dies von großer Bedeutung.
* **Skalierbarkeit**: Rust unterstützt die Entwicklung von skalierbaren Systemen durch die Möglichkeit, nativen Code auszuführen, der auf verschiedenen Plattformen, einschließlich Cloud-Infrastrukturen und verteilten Systemen, lauffähig ist.
* **Strenge Typisierung**: Rust verwendet ein strenges Typsystem, das dazu beiträgt, Fehler in der Programmlogik frühzeitig zu erkennen und die Codequalität zu erhöhen.

Durch die Verwendung von Rust als Programmiersprache wird ein solides Fundament für die Entwicklung eines leistungsstarken, sicheren und skalierbaren Simulationssystems geschaffen.


## Strukturen und Fuktionen

Dieses Schaubild zeigt die Aufteilung der im ersten Kapitel beschriebenen Komponenten auf sechs Module. Im Folgenden wird erläutert, wie es zu dieser Aufteilung kommt und wie die Module aufgebaut sind.

![Programmstruktur nach Klassen](docs/paper/assets/programmstruktur-klassen.png)

### `main.rs` - Die Kommandozentrale
Die `main.rs`-Datei ist die Hauptdatei eines Rust-Projekts und dient als Einstiegspunkt für die Ausführung des Programms. Sie enthält die `main`-Funktion, die beim Start des Programms aufgerufen wird. Die `main.rs`-Datei ist verantwortlich für die Initialisierung und Koordination der verschiedenen Komponenten und Module des Projekts sowie für die Steuerung des Programmablaufs und die Ausgabe von Ergebnissen.

### `train.rs` - Mehr als nur ICEs
Dieses Modul definiert verschiedene Strukturen und Funktionen für einen Zug, der aus mehreren Waggons besteht, die wiederum aus Sitzreihen und -segmenten bestehen. Der Zug hat eine Route mit verschiedenen Haltestellen. Der Code enthält auch eine Struktur für Sitzgruppen und deren Koordinaten.

#### `Train` - Ein Zug
In der Train-Struktur werden die grundlegenden Informationen zum Zug wie seine ID, die Koordinaten seiner Waggons und die Abmessungen seiner Sitzreihen und -segmente gespeichert. Die Route wird ebenfalls gespeichert. Die Funktion new generiert den Zug mit seinen Waggons und Sitzreihen und -segmenten.

``` Rust
pub struct Train {
    id: String,                         // Train id (e.g. "ICE 608")
    base_coordinates: (f64, f64),       // (x, y)
    coach_dimensions: (f64, f64),       // Dimensions of one coach (x, y)
    coaches: Vec<Coach>,                // [coach]
    route: Route,                       // Route
}
```

#### `Coach` - Ein Wagon
Die Coach-Struktur enthält die Informationen zu einem einzelnen Waggon, einschließlich seiner Nummer, der Koordinaten und einer Liste von Sitzreihen. Die Funktion seat_groups erstellt Sitzgruppen aus den Sitzreihen.
``` Rust
pub struct Coach {
    number: i32,                        // Coach number (e.g. 1, 2, 3, ...)
    base_coordinates: (f64, f64),       // (x, y - relative to train base coordinates)
    rows: Vec<Row>,                     // List of seat rows
}
```

#### `Row` - Sitzreihe
Die Row-Struktur enthält eine ID und eine Liste von Sitzreihen-Segmenten.
``` Rust
struct Row {
    id: i32,                            // Row id (e.g. 1, 2, 3, ...)
    segments: Vec<RowSegment>,          // List of row segments
}
```

#### `RowSegment` - Sitzreihensektion
Die RowSegment-Struktur enthält eine ID, die Nummer der Sitzreihe, die Seite (links oder rechts) und die Orientierung (vorwärts oder rückwärts). Sie enthält auch eine Liste von Sitzen.
``` Rust
struct RowSegment {
    id: i32,                            // Row segment id (e.g. 1, 2, 3, ...)
    row_no: i32,                        // Row number (e.g. 1, 2, 3, ...)
    side: Side,                         // Left or Right from aisle
    orientation: Orientation,           // Forward or Backward
    seats: Vec<Seat>,                   // List of seats
}
```

#### `Seat` - Sitzplatz
Ein Sitzplatz stellt die kleinste Einheit in einem Zug dar.
Die Seat-Struktur enthält die Informationen zu einem einzelnen Sitz, einschließlich seiner ID, Nummer, Koordinaten, Abmessungen, Typ (Fenster oder Gang), Klasse (erste oder zweite), Orientierung (vorwärts oder rückwärts), Abstand zum nächsten Ausgang und Abstand zum nächsten Speisewagen.
``` Rust
pub struct Seat {
    id: i32,                            // Seat id (e.g. 1001, 1002, 1003, ...)
    number: i32,                        // Seat number (e.g. 1, 2, 3, ...)
    base_coordinates: (f64, f64),       // (x, y - relative to coach base coordinates)
    dimensions: (f64, f64),             // (x, y)
    seat_type: SeatType,                // Window or Aisle
    limited_view: bool,                 // if seat is next to a window
    class: SequenceClass,               // First or Second class
    orientation: Orientation,           // Forward or Backward relative to the train
    distance_to_exit: f64,              // Distance to the nearest exit
    distance_to_dining: f64,            // Distance to the nearest dining car
}
```

Die `Route`-Struktur enthält eine Liste von Haltestellen und Funktionen zum Erstellen eines zufälligen Routensegments.

Die `RouteSegment`-Struktur enthält die Start- und Endstation eines Routensegments.

Die `SeatGroup`-Struktur enthält die ID und eine Liste von Sitz-IDs, die zu einer Gruppe gehören. Die Funktion center_coordinates berechnet die Koordinaten des Mittelpunkts einer Sitzgruppe.



#### Sitzgruppe
``` Rust
pub struct SeatGroup {
    id: i32,
    pub seats: Vec<i32>,
}
```

Die Struktur `SeatGroup` ist wichtig, da sie eine Gruppierung von Sitzplätzen innerhalb eines Wagens repräsentiert, die aufgrund ihrer räumlichen Nähe und ihrer Zugewandtheit zueinander als Sitzgruppe betrachtet werden. Die `SeatGroup` wird in der `board()`-Funktion verwendet, um die Sitzplatzwahl eines Passagiers zu optimieren, indem die am besten geeignete Sitzgruppe basierend auf den Präferenzen des Passagiers und der aktuellen Zugstruktur ausgewählt wird. Sie speichert anstelle der gesamten Daten zu den Sitzplätzen lediglich einen Verweis auf die Plätze in der `Train`-Struktur in Form einer einzigartigen `seat_id`.

#### Funktion: `new()`

Die Funktion `new()` ist ein Konstruktor der `Train`-Klasse, der ein neues `Train`-Objekt erstellt. Die Funktion nimmt drei Parameter entgegen: `coach_count`, `coach_size` und `route`. Der Parameter `coach_count` gibt die Anzahl der Waggons im Zug an, `coach_size` bestimmt die Anzahl der Sitzreihen in jedem Waggon, und `route` ist ein `Route`-Objekt, das die Zugroute repräsentiert. Die Funktion generiert dann den Zug mit den gegebenen Parametern, erstellt Waggons und Sitzreihen, bestimmt die Sitzpositionen und -eigenschaften und fügt sie entsprechend in die Struktur des Zuges ein.

### Modul: `passenger.rs` - Der Mensch
Die Passagier-Komponente ist ein wichtiger Bestandteil des gesamten Simulationssystems. Sie modelliert das Verhalten von Passagieren innerhalb des Zuges und ermöglicht die Analyse der Auswirkungen von verschiedenen Verkehrs- und Betriebsbedingungen auf die Passagiere. In diesem Kapitel wird die Struktur der Passagier-Komponente erläutert.

#### Klasse `Passenger`

Die Hauptklasse der Passagier-Komponente ist die `Passenger`-Klasse. Sie repräsentiert einen einzelnen Passagier und speichert Informationen über dessen Zustand, wie zum Beispiel den aktuellen Sitzplatz, die gewünschte Route und die start_position innerhalb des Zuges. Die Klasse bietet Methoden zur Simulation des Verhaltens eines Passagiers, wie das Einsteigen und Aussteigen aus dem Zug, das Wechseln von Sitzplätzen und das Suchen nach freien Sitzplätzen.

Die wichtigsten Attribute der `Passenger`-Klasse sind:

- `id`: Die eindeutige Identifikationsnummer des Passagiers.
- `route_segment`: Das gewünschte Fahrtsegment des Passagiers, bestehend aus Start- und Zielstation.
- `start_position`: Die relative Startposition des Passagiers innerhalb des Zuges, angegeben als (x, y)-Koordinate.
- `wish_to_seat`: Ein boolescher Wert, der angibt, ob der Passagier einen Sitzplatz bevorzugt oder stehen möchte.
- `seat`: Die ID des aktuellen Sitzplatzes des Passagiers. Wenn der Wert 0 ist, steht der Passagier.

Die Hauptmethoden der `Passenger`-Klasse sind:

- `new()`: Erstellt ein neues `Passenger`-Objekt mit den angegebenen Attributen.
- `board()`: Lässt den Passagier in den Zug einsteigen und einen Sitzplatz oder Stehplatz wählen.
- `sit()`: Lässt den Passagier auf dem angegebenen Sitzplatz Platz nehmen.
- `exit()`: Lässt den Passagier aus dem Zug aussteigen und gibt den aktuellen Sitzplatz frei.

#### Interaktion mit der Klasse `Train`

Die Passagier-Komponente interagiert eng mit der Train-Komponente, um das Verhalten von Passagieren im Kontext des Zuges zu simulieren. Die `Passenger`-Klasse verwendet Informationen über den Zug, wie zum Beispiel die Anzahl und Anordnung der Wagen und Sitzgruppen, um Entscheidungen über das Ein- und Aussteigen, das Wechseln von Sitzplätzen und das Suchen nach freien Sitzplätzen zu treffen.

Ein wesentlicher Aspekt dieser Interaktion ist die Berücksichtigung der räumlichen Dimensionen des Zuges, um die relative Startposition eines Passagiers innerhalb des Zuges zu berechnen und die Entfernungen zwischen verschiedenen Sitzgruppen und Türen im Zug zu bestimmen. Dies ermöglicht es, die Passagierbewegungen innerhalb des Zuges auf eine realistische Weise zu simulieren. <!-- und die Auswirkungen von verschiedenen räumlichen Faktoren, wie zum Beispiel der Anordnung von Sitzplätzen und Türen, auf das Passagierverhalten zu analysieren. -->

#### Funktion: `board()`

``` Rust
board(&mut self, train: &Train, passengers: &Vec<Passenger>)
```

Die `board()`-Funktion ist eine zentrale Methode in der `Passenger`-Klasse, die den Einsteigeprozess eines Passagiers in den Zug simuliert. Diese Funktion ermöglicht es, den Passagier entsprechend seiner Präferenzen und der aktuellen Zugstruktur einen Sitzplatz oder Stehplatz wählen zu lassen. In diesem Abschnitt wird die Funktionsweise der `board()`-Funktion im Detail erläutert.

Die `board()`-Funktion wird aufgerufen, wenn ein Passagier in den Zug einsteigen möchte. Sie nimmt zwei Parameter entgegen:

- `train`: Ein Referenzobjekt der `Train`-Klasse, das den aktuellen Zustand des Zuges repräsentiert.
- `passengers`: Eine Referenz auf eine Liste von `Passenger`-Objekten, die die aktuell im Zug befindlichen Passagiere repräsentiert.

Der Einsteigeprozess besteht aus mehreren Schritten, die nacheinander ausgeführt werden:


Die Bewertung der Sitzgruppen erfolgt anhand der Verfügbarkeit von Sitzplätzen und der Entfernung der Sitzgruppe zur Startposition des Passagiers in Relation zur Länge des Wagens. Die Liste der bewerteten Sitzgruppen wird dann nach der Verfügbarkeit von Sitzplätzen gefiltert und nach Bewertung sortiert.

``` Rust
// Filter seat groups by capacity
seat_group_evaluations.retain(|seat_group_evaluation| seat_group_evaluation.1 > 0);

if seat_group_evaluations.len() > 0 {

    // Sort seat groups by evaluation
    seat_group_evaluations.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    ...
}
```

Nun wählt der Passagier die beste Sitzgruppe, welche das erste Element der nach Bewertung sortierten Liste ist.

```Rust
// Choose seat group
let seat_group_id = seat_group_evaluations[0].0;
```

Schließlich wählt der Passagier einen freien Sitzplatz in der ausgesuchten Sitzgruppe aus.

### `data.rs` - Schnittstelle zum Zug
Um eine Vorhersage darüber zu treffen, welche Plätze in einem Zug wahrscheinlich frei sind, werden verschiedene Daten aus dem Zug gesammelt und verwaltet. Dazu gehören Informationen zu WLAN-Geräten, Reservierungen, Komfort Check-Ins und kontrollierten Fahrgästen. 

Die Input-Daten werden in verschiedenen Datenstrukturen abgelegt. Die WiFi-Daten beinhalten Informationen über die Signalstärke der Router in den verschiedenen Zugwaggons. Die Reservierungsdaten enthalten Angaben zu den reservierten Sitzen auf den verschiedenen Streckenabschnitten. Die Daten zum Komfort Check-In geben Auskunft darüber, welche Sitze von den Passagieren für den Komfort Check-In ausgewählt wurden. Die Daten zu kontrollierten Fahrgästen enthalten Informationen darüber, welche Sitze von Kontrolleuren überprüft wurden. 

Die Verwaltung der Input-Daten erfolgt durch die Strukturen Data, WiFiData, ReservationData, KomfortCheckInData und CheckedPassengersData. Die Struktur Data dient dabei als übergeordnete Struktur, welche alle anderen Datentypen umfasst.

Die Daten werden in der Simulation mitheilfe eines Generators aus der Verteilung der Fahrgäste im Zug und weiterer Parameter, den $k$-Werten, errechnet.

### `request.rs` - Schnittstelle zum Client
Im Modul `request.rs` wird eine Schnittstelle zur Entgegennahme von Anfragen definiert. Die Anfragen enthalten Informationen zu den vom Nutzer (Client) gewünschten Sitzplätzen, wie beispielsweise Fensterplatz, Nähe zum Ausgang oder zur Speisewagen. Zudem wird der Zug identifiziert, in dem die Platzsuche stattfinden soll, sowie das Segment der Zugstrecke, auf dem sich der Client befindet. Die Klasse des Zuges wird ebenfalls angegeben.

Die Implementierung umfasst die Definition von Datenstrukturen wie `Request` und `SeatRequirements`, die die Anfrageparameter speichern. Eine Beispielanfrage wird durch die Funktion `example()` erstellt.


### `simulation.rs` - Der Zug. In Bewegung.
Das Modul `simulation.rs` dient der Generierung von Daten, indem es eine Zugfahrt simuliert. Die Struktur `Journey` repräsentiert eine Zugfahrt, die simuliert werden soll. Die Methode `simulate()` nimmt drei Parameter entgegen: einen `Train`, eine Anzahl von Passagieren `passengers_count` und eine `wish_to_seat_chance`, die die Wahrscheinlichkeit angibt, dass ein Passagier sich hinsetzen möchte. In dieser Methode werden Passagiere generiert, die zufällig an verschiedenen Haltestellen ein- und aussteigen (Verhalten bei der Platzwahl: siehe `passenger.rs`). An jeder Haltestelle wird auch eine Aufzeichnung erstellt, die die Anzahl der Passagiere und ihre Platzbelegung enthält. Die gesamte Zugfahrt wird als `Journey` Struktur zurückgegeben.

Die Methode `save_to_file()` speichert die generierten Daten als JSON-Datei.

Die Struktur `Stop` enthält Informationen über eine Haltestelle, einschließlich der Platzverteilung im Zug nach dem Halt. Diese Informationen werden von der `simulate()` Methode generiert und in jeder Haltestelle aufgezeichnet.

Diese Herangehensweise hat mehrere Vorteile:

1. **Testbarkeit**: Durch die Simulation von verschiedenen Szenarien kann die Leistung des Systems getestet und verbessert werden.

2. **Anpassbarkeit**: Der Code ermöglicht es dem Benutzer, verschiedene Parameter wie die Anzahl der Passagiere und ihre Verhaltensmuster anzupassen, um verschiedene Szenarien zu simulieren und die Leistung des Systems in verschiedenen Situationen zu testen.

3. **Datenerfassung**: Die Simulation erfasst wichtige Daten wie die Anzahl der Passagiere und ihre Platzbelegung, die zur Optimierung des Betriebs genutzt werden können.


<!-- TODO: Wird im nächsten Kapitel genauer erläutert 
### `allocator.rs`
Weist einen Platz zu
Sicherheitsfaktor wägt ab zwischen
  Wahrscheinlichkeit für Platz, frei zu sein
  Erfüllung der benutzerdefinierten Anforderungen an den Platz

#### Wahrscheinlichkeitsgenerator

Der Wahrscheinlichkeitsgenerator errechnet anhand der Daten des Datengenerators die Wahrscheinlichkeiten der Belegtheit der Plätze.
-->



## Simulation
<!-- TODO: Hier Ausführung der Simulation und Ergebnisse erläutern -->

### Der Zug
Zunächt wird die Reisestrecke festgelegt. Hierzu wird die Beispielstrecke verwendet.
Als kleines Beispiel wird ein Zug mit zwei Waggons und der Waggongröße 5 generiert.

``` Rust
let route = Route::example();
let train = Train::new(2, 5, route);
```

Folgendes ist die Fartstrecke:

``` Rust
route: Route {
    stops: [
        "Freiburg",
        "Karlsruhe",
        "Mannheim",
        "Berlin",
        "Hamburg",
    ],
},
```

Der generierte Zug ist verfügbar unter: <https://gist.github.com/theKevinKretz/8149ba97c0c1d9cf9dc4abf0723f615e>


### Die Reise
Es wird eine Reise mit dem oben definierten Zug und 20 Passagieren simuliert.
Hierzu wird in der `main()`-Funktion folgender Code ausgeführt:

``` Rust
let simulation = Journey::simulate(&train, 20, 0.8);
```

Folgendes sind die Ergebnisse (betrachtet werden die Passagiere 5 und 6 an den Haltestellen "Freiburg", "Karlsruhe", und "Mannheim".):

``` Rust
{
  "station": "Freiburg",
  "passengers": [
    // ...
    {
      "id": 5,
      "route_segment": {
        "start_station": "Karlsruhe",
        "end_station": "Berlin"
      },
      "start_position": [
        -8.150504046354957,
        89.6232474341507
      ],
      "wish_to_seat": true,
      "seat": 0
    },
    {
      "id": 6,
      "route_segment": {
        "start_station": "Freiburg",
        "end_station": "Mannheim"
      },
      "start_position": [
        -9.33531507513891,
        15.98733632802289
      ],
      "wish_to_seat": true,
      "seat": 1001
    },
    // ...
  ]
},
{
  "station": "Karlsruhe",
  "passengers": [
    // ...
    {
      "id": 5,
      "route_segment": {
        "start_station": "Karlsruhe",
        "end_station": "Berlin"
      },
      "start_position": [
        -8.150504046354957,
        89.6232474341507
      ],
      "wish_to_seat": true,
      "seat": 2008
    },
    {
      "id": 6,
      "route_segment": {
        "start_station": "Freiburg",
        "end_station": "Mannheim"
      },
      "start_position": [
        -9.33531507513891,
        15.98733632802289
      ],
      "wish_to_seat": true,
      "seat": 1001
    },
    // ...
  ]
},
{
  "station": "Mannheim",
  "passengers": [
    // ...
    {
      "id": 5,
      "route_segment": {
        "start_station": "Karlsruhe",
        "end_station": "Berlin"
      },
      "start_position": [
        -8.150504046354957,
        89.6232474341507
      ],
      "wish_to_seat": true,
      "seat": 2008
    },
    {
      "id": 6,
      "route_segment": {
        "start_station": "Freiburg",
        "end_station": "Mannheim"
      },
      "start_position": [
        -9.33531507513891,
        15.98733632802289
      ],
      "wish_to_seat": true,
      "seat": 0
    },
    // ...
  ]
},
// ...
```
Vollständige Simulation unter: <https://gist.github.com/theKevinKretz/1fab1bfe85df9ef22f2258adf84bae55>

In den Ergebnissen ist zu sehen, dass einige Passagiere sich dafür entschieden haben, zu stehen, während andere einen Sitzplatz bevorzugten. Die Passagiere, die sich für einen Sitzplatz entschieden haben, haben auch unterschiedliche Sitzplätze gewählt. Sie steigen außerdem an den richtigen Haltestellen ein und aus.

Die Simulation ist Grundlage für die weitere Arbeit.


<!--
# Kapitel III: Platzanweiser / Optimierung
Es wrid davon ausgegangen, dass sich die Plätze, optimaler Weise innerhalb einer Sitzgruppe befinden sollen.
-->


<!--
# Kapitel IV: Verhaltenspsychologie

Persönliche Fehlertoleranz
  Sicherheitsfaktor
Konzept für mobile App
Benutzerfreundlichkeit
  Anfragen stellen
  Ergebnisse aufbereiten
Performanz

## Persönliche Fehlertoleranz

### Genauigkeit

### Transparen
-->

# Quellen- und Literaturverzeichnis