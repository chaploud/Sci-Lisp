;; === Keywords
;; Must
false
true
nil
if
when
cond
import  ;; Don't confuse users
def
defn
fn
for
while
range
break
continue
try
catch
throw
return
and
or
xor
not
del
is
+
++
-
--
*
/
//
>
>=
<
<=
=
==
apply
ns
assoc
assoc!
assoc-in
atom
assert
head
tail
sym-to-str
str-to-sym
chunk
class?
string?
int?(i8?, i16?, ...)
float?(f32?, f64?)
coll?
comment
comp
cond
conj
conj!
cons
contains?(in?)
count(len/size)
cycle
struct
declare
method
let
format
const
dissoc
doseq
empty?
even?
odd?
enumerate
push
pop
first
last
false?
true?
filter
map
find
flatten
zip
fn?
group-by
if-let
juxt
into
keys
values
list
macroexpand
macroexpand-1
max
min
log
log10
sin
cos
tan
atan
acos
merge
mod
nan?
neg?
pos?
new
print
printf
nil?
inf?
-inf?
any?
some?
every?
not-any?
not-empty?
not-every?
partial
quot
rem
quote
rand
rand-int
read
write
remove
repeat
replace
rest
reverse
second
seq
seq?
shuffle
sort
sort-by
str
swap!
sym
sym?
time
type
update
with-open
zero?
enum
private
protected
public


;; Want
async
await
in (contain)
with
pass
as
yield
switch, case

;; Choice

;; Functions
vector
array
list
string

;; Scientific/Math
sum
mean
std
count/size/len
abs
cos
sin
tan

;; Types (Literal)
sym          ; symbol :blur
bool         ; true/false
i8           ; 100i8    i8 is optional
i16          ; 100i16   i16 is optional
i32          ; 100i32   i32 is optional
i64          ; 100i64   i64 is optional
i128         ; 100i128  i128 is optional
u8           ; 100u8    u8 is optional
u16          ; 100u16   u16 is optional
u32          ; 100u32   u32 is optional
u64          ; 100u64   u64 is optional
u128         ; 100u128  u128 is optional
f32          ; 8.6f32   f32 is optional
f64          ; 8.6f64   f64 is optional
ibig         ; 10000ibig arbitrary precision integer(can use N literal)
fbig         ; 3.141fbig arbitrary precision float(can use M literal)
str          ; string  "lisp"
regex        ; Regular Expression #"[0-9]+"
list         ; linked list '(1, 2, 3) allow any type combination
v<T>[i,j,k]  ; vector [2.0, 3.0, 4.0]:v<f64>[3]
a<T>[i,j,k]  ; array [[1, 2], [3, 4], [5, 6]]:a<i32>[2,3]
m<K,V>       ; HashMap {:a 1, :b 2, :c 3}:m<sym,i32>
s<K>         ; HashSet #{"a", "b", "c"}:s<str>

;; === Syntax
(def a:bool true)
(def a:sym :blur)
(def a:i32 36)
(def a:f64 3.1415)
(def a:ibig 36N)
(def a:fbig 3.14M)
(def a:str "abc")
(def a:regex #"[0-9]+")
(def a:list '(:a, 1, {:a 2}, [[1, 2], [3, 4]], ("a" "b" :a))) ; difficult
(def a:v<i32>[3] [1, 2, 3])
(def a:a<f64>[2,2] [[1, 2], [3, 4]])
(def a:m<sym,str> {:a "a", :b "b"})
(def a:s<sym> #{:a, :b, :c})

;; Index slicing
; TODO: nested generics is hard to read
; inspired by numpy
(def vec:v<i32>[3,2] [[1, 2], [3, 4], [5, 6]])
([0] vec)
=> [1, 2] :v<i32>
([-1] vec)
=> [5, 6]
([0:-1] vec)
=> [[1, 2], [3, 4]]
([0, 1] vec)
=> 2
([:, 1] vec)
=> [2, 4, 6]


(def hashmap:m<sym,str> {:a "a", :b "b"})
([:a] hashmap)
=> "a"

;; Funciton

;; Class

;; Macro

;; RegExp
(find #"[0-9]+" "aa123a")
=> "123"

(match #"hello, (.*)" "hello, world")
=> ["hello, world", "world"]
