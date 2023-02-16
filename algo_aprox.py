#this python program that is an aprroximation of the algorith im going to use
#the point of this program is to approximate how many words will end up ine each data set
#if i use the general algorithim defined in planning.txt

#the goal is to find what iteration of this algorithm will create the most even spread of data
a=0
b=0
c=0
d=0
e=0
f=0
g=0
h=0
i=0
j=0
k=0
l=0
m=0
n=0
o=0
p=0
q=0
r=0
s=0
t=0
u=0
v=0
w=0
x=0
y=0
z=0

file = open('Collins_Scrabble_Words_2019.txt','r')

for word in file:
    word=word.casefold()
    if(word.__contains__('q')):
        q+=1
    elif(word.__contains__('z')):
        z+=1
    elif(word.__contains__('j')):
        j+=1
    elif(word.__contains__('x')):
        x+=1
    elif(word.__contains__('k')):       #maybe move this down but k has a high value
        k+=1
    elif(word.__contains__('f')):
        f+=1
    elif(word.__contains__('w')):
        w+=1
    elif(word.__contains__('v')):
        v+=1
    elif(word.__contains__('h')):
        h+=1
    elif(word.__contains__('y')):
        y+=1
    elif(word.__contains__('b')):
        b+=1
    elif(word.__contains__('m')):
        m+=1
    elif(word.__contains__('g')):
        g+=1
    elif(word.__contains__('p')):
        p+=1
    elif(word.__contains__('c')):
        c+=1
#    elif(word.__contains__('d')):
#        d+=1
#    elif(word.__contains__('u')):
#        u+=0
#    elif(word.__contains__('l')):
#        l+=1
#    elif(word.__contains__('o')):
#        o+=1
#    elif(word.__contains__('n')):
#        n+=1
#    elif(word.__contains__('t')):
#        t+=1
#    elif(word.__contains__('i')):
#        i+=1
#    elif(word.__contains__('r')):
#        r+=1
#    elif(word.__contains__('a')):
#        a+=1
#    elif(word.__contains__('s')):
#        s+=1
    else:
        e+=1
        
print("q=",q)
print("z=",z)       
print("j=",j)
print("x=",x)
print("k=",k)
print("f=",f)
print("w=",w)
print("v=",v)
print("h=",h)
print("y=",y)
print("b=",b)
print("m=",m)
print("g=",g)
print("p=",p)
print("c=",c)
print("d=",d)
print("u=",u)
print("l=",l)
print("o=",o)
print("n=",n)
print("t=",t)
print("i=",i)
print("r=",r)
print("a=",a)
print("s=",s)
print("1 pt=",e)