program hello_user
    implicit none
    
    integer :: x, y, z, i, unit
    integer :: the_squares(10)
    integer, parameter :: inches = 12 !paramter: named constant
    character(len=20) :: name

    print *, inches

    x = 3
    y = 5
    z = (sin(0.5 - 0.5)*x + fib(5)*y - 2**4)/5
    print *, z

    select case (1 + 2)
    case (2 + 1, 2 + 2)
        print *, "was 3 or 4"
    end select

    select case ("Hi")
    case ("Hello")
        print *, "Hello"
    case default
        print *, "Default"
    end select

    do i = 10, 1, -2
        print *, i
    end do

    i = 100
    do while (i > 0)
        print *, "Can you guess?"
        i = i / 2
    end do

    the_squares = 42
    print *, the_squares

    do i = 1, 10
        the_squares(i) = i**2
    end do
    
    do i = 1, 10
        print *, the_squares(i)
    end do
    
    print *, the_squares
    print *, the_squares(2:4)

    select case (command_argument_count())
    case (0)
        print *, "What is your name?"
        read(*, '(A)') name
    case (1) 
        call get_command_argument(1, name)
    case default
        print *, "Too many command line arguments!"
        print *, "Usage: hello_user [name]"
        stop 1
    end select

    print *, "Hello, " // trim(name) // "!"

    open(newunit=unit, file="hello.txt", status="replace")
    write(unit, '(A)') "Hello, " // trim(name) // "!!!"
    close(unit) 
contains
    pure recursive function fib(n) result(fib_)
        integer, intent(in) :: n
        integer :: fib_

        if (n == 1 .or. n == 2) then
            fib_ = 1
        else
            fib_ = fib(n - 1) + fib(n - 2)
        end if
    end function
end program
