math_dict = {'one': 1, 'two':2, 'three':3, 'four':4, 'twenty':20, 'thirty':30}



def word_replace(s:str):
    words = s.split(' ')
    new_words = []
    for word in words:
        w = math_dict.get(word)
        if w:
            new_words.append(w)
        else:
            new_words.append(word)
    result = new_add( reduction(new_words))
    return result

def reduction(w):
    result = []
    for i in range(len(w)):
        if i == 0:
            result.append(w[i])
        else:
            if type(w[i]) == int:
                if type(w[i-1]) == int:
                    result[-1] = result[-1] + w[i]
                else:
                    result.append(w[i])
            else: 
                result.append(w[i])
    return result
                

                
def new_add(s):
    if s[1] == 'add':
        result = s[0]+s[2]
    else:
        result = -1
    return result
            



if __name__ == '__main__':
    result = word_replace("one add thirty three")
    print(result)



