import java.util.Stack;

public class Basic {
    public static void main(String[] args) {
        int i = 010;
        int j = 07;
        System.out.println(i);
        System.out.println(j);

        try {
            Float f1 = new Float("3.0");
            int x = f1.intValue();
            byte b = f1.byteValue();
            double d = f1.doubleValue();
            System.out.println(x + b + d);
        } catch (NumberFormatException e) {
            System.out.println("Bad number");
        }
    }

//    class Adder extends Calculator {
//        @Override
//        int add(int a, int b) {
//            return a + b;
//        }
//    }

    public static class Parser {
        public static boolean isBalanced(String s) {
            Stack<Character> stack = new Stack<Character>();
            for(int i = 0; i < s.length(); i++) {
                char c = s.charAt(i);
                if(c =='[' || c == '(' || c == '{') {
                    stack.push(c);
                }else if(c == ']') {
                    if(stack.isEmpty() || stack.pop() != '[') {
                        return false;
                    }
                }else if(c == ')') {
                    if(stack.isEmpty() || stack.pop() != '(') {
                        return false;
                    }
                }else if(c == '}') {
                    if(stack.isEmpty() || stack.pop() != '{') {
                        return false;
                    }
                }
            }
            return stack.isEmpty();
        }
    }
}
