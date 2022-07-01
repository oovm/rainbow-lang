{scope}
key = "scope.key"

    // Although indentation is not necessary, it is indented four spaces for readability.
    {.inherit1.sub1}
    key = "scope.inherit1.sub1.key"

    {.inherit1.sub2}
    key = "scope.inherit1.sub2.key"

        // Whenever there is inheritance indented by four
        {..inherit2.sub1}
        key = "scope.inherit1.sub2.inherit2.sub1.key"

        {..inherit2}
        sub2.key = "scope.inherit1.sub2.inherit2.sub2.key"
        sub3 = {
            key = "scope.inherit1.sub2.inherit2.sub3.key"
        }

        // This field can not be generated
        {..empty}

    {.inherit3}
    sub1.key = "scope.inherit3.sub1.key"

    {.inherit4.sub1}
    key = "scope.inherit4.sub1.key"

    {.empty}

{another}
key = "another.key"

{empty}